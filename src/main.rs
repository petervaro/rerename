use std::{
    io,
    error,
    path::Path,
    fs::rename,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

use clap::{
    App,
    Arg,
    ArgMatches,
};

use regex::{
    self,
    Regex,
};


/*----------------------------------------------------------------------------*/
const LICENSE: &'static str = "\
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
#[derive(Debug)]
enum Error
{
    IoError(io::Error),
    RegexError(regex::Error),
}


/*----------------------------------------------------------------------------*/
impl error::Error for Error {}


/*----------------------------------------------------------------------------*/
impl Display for Error
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        use Error::*;

        match self
        {
            IoError(error) => write!(f, "{}", error),
            RegexError(error) => write!(f, "{}", error),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl From<io::Error> for Error
{
    fn from(error: io::Error) -> Self
    {
        Self::IoError(error)
    }
}


/*----------------------------------------------------------------------------*/
impl From<regex::Error> for Error
{
    fn from(error: regex::Error) -> Self
    {
        Self::RegexError(error)
    }
}


/*----------------------------------------------------------------------------*/
fn arguments<'a>() -> ArgMatches<'a>
{
    let dry_run =
        Arg::with_name("dry_run").short("d")
                                 .long("dry-run")
                                 .takes_value(false);

    let source =
        Arg::with_name("source").short("s")
                                 .long("source")
                                 .takes_value(true)
                                 .required(true)
                                 .help("Matching pattern");

    let target =
        Arg::with_name("target").short("t")
                                 .long("target")
                                 .takes_value(true)
                                 .required(true)
                                 .help("Replace pattern (groups referenceed as \
                                        `$N` (e.g. `$1` or `$2`) or `$name` if \
                                        `(?P<name>)` is used");

    let file_names =
        Arg::with_name("file_names").takes_value(true)
                                    .multiple(true)
                                    .index(1)
                                    .required(true);

    App::new("rerename").version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .arg(dry_run)
                        .arg(source)
                        .arg(target)
                        .arg(file_names)
                        .after_help(LICENSE)
                        .set_term_width(80)
                        .get_matches()
}


/*----------------------------------------------------------------------------*/
fn dumb_rename<P: AsRef<Path>, Q: AsRef<Path>>(_: P, _: Q) -> io::Result<()> { Ok(()) }


/*----------------------------------------------------------------------------*/
fn main() -> Result<(), Error>
{
    let arguments = arguments();
    let source = Regex::new(arguments.value_of("source").unwrap())?;
    let target = arguments.value_of("target").unwrap();
    let rename =
        if arguments.is_present("dry_run") { dumb_rename } else { rename };

    let mut renamed = 0usize;
    for old_name in arguments.values_of("file_names").unwrap()
    {
        let new_name = source.replace(old_name, target);
        if old_name != new_name
        {
            println!("{} -> {} ", old_name, &new_name);
            rename(old_name, new_name.to_string())?;
            renamed += 1;
        }
    }

    println!("Renamed {} file(s)", renamed);
    Ok(())
}
