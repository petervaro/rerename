use std::hint::unreachable_unchecked;

use regex::Regex;

use lazy_static::lazy_static;


/*----------------------------------------------------------------------------*/
lazy_static!
{
    static ref SPLITTER: Regex =
        Regex::new(r"(?P<word>\w+)|(?P<else>\W+)").unwrap();
}


/*----------------------------------------------------------------------------*/
// TODO: Somehow find a way to only allocate a single
//       `String` for all inputs and all transformations
type Transformer = fn(&str) -> String;


/*----------------------------------------------------------------------------*/
pub struct Transformers(Vec<Transformer>);


/*----------------------------------------------------------------------------*/
impl Transformers
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub const UPPERCASE: &'static str = "upper";

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub const LOWERCASE: &'static str = "lower";

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub const TITLECASE: &'static str = "title";

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub const CAPITALISED: &'static str = "capital";

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn to_titlecase(input: &str) -> String
    {
        let mut output = String::new();

        for captured in SPLITTER.captures_iter(input)
        {
            if let Some(matched) = captured.name("word")
            {
                let mut characters = matched.as_str().chars();
                output.extend(characters.next()
                                        .unwrap()
                                        .to_uppercase()
                                        .chain(characters));
            }
            else if let Some(matched) = captured.name("else")
            {
                output.push_str(matched.as_str());
            }
        }

        output
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn to_capitalised(input: &str) -> String
    {
        let mut output = String::new();
        let mut characters = input.chars();
        output.extend(characters.next()
                                .unwrap()
                                .to_uppercase()
                                .chain(characters));
        output
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn transformer(transformer: &str) -> Transformer
    {
        match transformer
        {
            Self::UPPERCASE => str::to_uppercase,
            Self::LOWERCASE => str::to_lowercase,
            Self::TITLECASE => Self::to_titlecase,
            Self::CAPITALISED => Self::to_capitalised,
            other =>
            {
                debug_assert!(false, "Invalid transformer: {}", other);
                unsafe { unreachable_unchecked() }
            }
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn apply(&self, string: &str) -> String
    {
        self.0.iter().fold(string.into(), |s, transformer| transformer(&s))
    }
}


/*----------------------------------------------------------------------------*/
impl<'a, I> From<I> for Transformers
    where I: Iterator<Item = &'a str>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(transformers: I) -> Self
    {
        Self(transformers.map(Self::transformer).collect())
    }
}
