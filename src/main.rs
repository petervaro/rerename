use std::{
    io,
    path::Path,
    fs::rename,
    cmp::Ordering,
};

use clap::{
    App,
    Arg,
    ArgMatches,
};

use regex::Regex;

use rerename::{
    self,
    FileNames,
    Referencer,
    Converter,
    Formatter,
    Variables,
};


/*----------------------------------------------------------------------------*/
const LICENSE: &str = "\
LICENSE:
    Copyright (C) 2020 Peter Varo

    This program is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    This program is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License along with
    this program.  If not, see <https://www.gnu.org/licenses/>.
";


/*----------------------------------------------------------------------------*/
fn arguments<'a>() -> ArgMatches<'a>
{
    let dry_run =
        {
            let help =
                "Prints out the changes but does \
                 not commit them to the filesystem";

            Arg::with_name("dry_run").short("d")
                                     .long("dry-run")
                                     .takes_value(false)
                                     .help(help)
        };

    let source =
        {
            let help =
                "Matching pattern.  The pattern must use the features \
                 supported by this `Regex` implementation: \
                 https://docs.rs/regex/latest/regex/#syntax";

            Arg::with_name("source").short("s")
                                    .long("source")
                                    .alias("from")
                                    .takes_value(true)
                                    .required(true)
                                    .value_name("REGEX-PATTERN")
                                    .help(help)
        };



    let target =
        {
            let help =
                "Replace pattern (groups referenceed as `$N` (e.g. `$1` or \
                 `$2`) or `$name` if `(?P<name>)` was used in the matching \
                 pattern.  The special variable `@{index}` could be used to \
                 insert index number, which starts from `index-start` and \
                 increases on every file from the input, moreover it can take \
                 a format specifier for padding, e.g. `@{index:0>2}` which \
                 will produce indices as follows: 01, 02, .., 99, 100";

            Arg::with_name("target").short("t")
                                    .long("target")
                                    .alias("to")
                                    .takes_value(true)
                                    .required(true)
                                    .value_name("BACKREF-PATTERN")
                                    .help(help)
        };

    let index_start =
        {
            let help = "The `@{index}` variable's first value";
            Arg::with_name("index_start").short("i")
                                         .long("index-start")
                                         .takes_value(true)
                                         .default_value("0")
                                         .value_name("INDEX")
                                         .help(help)
        };

    let order_by =
        {
            let help =
                "Ordering the inputs based on the regex backreference (groups \
                 referenceed as `N` (e.g. `1` or `2`) or `name` if \
                 `(?P<name>)` was used in the matching pattern.  A special \
                 instruction could be given to convert the matched value into \
                 a type, so that the ordering will happen on that value \
                 instead of the matched string: `N:T` or `name:T`, where T can \
                 have the following values: `int` and `str` (which is the \
                 default type and implicitly implied)";

            Arg::with_name("order_by").short("o")
                                      .long("order-by")
                                      .takes_value(true)
                                      .value_name("GROUP[:TYPE]")
                                      .help(help)
        };

    let file_names =
        {
            let help = "The files the `source` pattern \
                        is going to be matched against";

            Arg::with_name("file_names").takes_value(true)
                                        .multiple(true)
                                        .index(1)
                                        .value_name("FILES")
                                        .required(true)
                                        .help(help)
        };

    App::new("rerename").version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .arg(dry_run)
                        .arg(source)
                        .arg(target)
                        .arg(order_by)
                        .arg(index_start)
                        .arg(file_names)
                        .after_help(LICENSE)
                        .set_term_width(80)
                        .get_matches()
}


/*----------------------------------------------------------------------------*/
fn stub_rename<P, Q>(_: P, _: Q) -> io::Result<()>
    where P: AsRef<Path>,
          Q: AsRef<Path>
{
    Ok(())
}


/*----------------------------------------------------------------------------*/
#[inline]
fn order<'a>(left: &str,
             right: &str,
             pattern: &Regex,
             referencer: &Referencer<'a>,
             converter: &Converter) -> Ordering
{
    use Ordering::*;

    match (pattern.captures(left), pattern.captures(right))
    {
        (Some(left), Some(right)) =>
            match (referencer.get(left), referencer.get(right))
            {
                (Some(left), Some(right)) =>
                {
                    let left = converter.to_comparable(left.as_str());
                    let right = converter.to_comparable(right.as_str());
                    match (left, right)
                    {
                        (Ok(left), Ok(right)) => left.cmp(&right),
                        (Ok(_), Err(_)) => Less,
                        (Err(_), Ok(_)) => Greater,
                        _ => Equal,
                    }
                },
                (Some(_), None) => Less,
                (None, Some(_)) => Greater,
                _ => Equal,
            },
        _ => Equal,
    }
}


/*----------------------------------------------------------------------------*/
fn main() -> rerename::Result<()>
{
    let arguments = arguments();
    let source = Regex::new(arguments.value_of("source").unwrap())?;
    let target = arguments.value_of("target").unwrap();
    let rename =
        if arguments.is_present("dry_run") { stub_rename } else { rename };
    let old_names = arguments.values_of("file_names").unwrap();

    let mut collected = Vec::new();
    let old_names: FileNames =
        match arguments.value_of("order_by")
        {
            Some(reference) =>
            {
                let reference = reference.split(':').collect::<Vec<_>>();
                let (reference, kind) =
                    match reference.as_slice()
                    {
                        [reference] => (*reference, "str"),
                        [reference, kind] => (*reference, *kind),
                        _ => return Err("Invalid order-by value".into()),
                    };

                let referencer =
                    match reference.parse::<u8>()
                    {
                        Ok(index) => index.into(),
                        Err(_) => reference.into(),
                    };

                let converter = Converter::new(kind)?;

                collected.extend(old_names);
                collected.sort_unstable_by(
                    |left, right|
                        order(left, right, &source, &referencer, &converter));
                collected.iter().into()
            },
            None => old_names.into(),
        };

    let index_start = arguments.value_of("index_start")
                               .unwrap()
                               .parse::<usize>()?;
    let mut variables = Variables::new(index_start);
    let formatter = Formatter::new(target, &variables)?;
    let mut target = String::new();
    let mut renamed = 0usize;
    let mut checked = 0usize;

    for old_name in old_names
    {
        target.clear();
        formatter.format(&variables, &mut target).unwrap();
        let new_name = source.replace_all(old_name, target.as_str());

        println!("{} -> {} ", old_name, &new_name);

        if old_name != new_name
        {
            rename(old_name, new_name.to_string())?;
            renamed += 1;
        }

        *variables.index_mut() += 1;
        checked += 1;
    }

    println!();
    println!("Checked {} file(s)", checked);
    println!("Renamed {} file(s)", renamed);
    Ok(())
}
