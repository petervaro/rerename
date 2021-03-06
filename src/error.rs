use std::{
    io,
    error,
    result,
    fmt::{
        self,
        Display,
        Formatter,
    },
    num::ParseIntError,
};


/*----------------------------------------------------------------------------*/
pub type Result<T> = result::Result<T, Error>;


/*----------------------------------------------------------------------------*/
#[derive(Debug)]
pub enum Error
{
    IoError(io::Error),
    RegexError(regex::Error),
    NumError(ParseIntError),

    SimpleStringError(String),
}


/*----------------------------------------------------------------------------*/
impl error::Error for Error {}


/*----------------------------------------------------------------------------*/
impl Display for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        use Error::*;

        match self
        {
            IoError(error) => write!(f, "{}", error),
            RegexError(error) => write!(f, "{}", error),
            NumError(error) => write!(f, "{}", error),
            SimpleStringError(error) => write!(f, "{}", error),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl From<io::Error> for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(error: io::Error) -> Self
    {
        Self::IoError(error)
    }
}


/*----------------------------------------------------------------------------*/
impl From<regex::Error> for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(error: regex::Error) -> Self
    {
        Self::RegexError(error)
    }
}


/*----------------------------------------------------------------------------*/
impl From<ParseIntError> for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(error: ParseIntError) -> Self
    {
        Self::NumError(error)
    }
}


/*----------------------------------------------------------------------------*/
impl From<String> for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(message: String) -> Self
    {
        Self::SimpleStringError(message)
    }
}


/*----------------------------------------------------------------------------*/
// TODO: Hopefully the insanely stupid compilation error about "forward
//       compatibility" (https://github.com/rust-lang/rfcs/issues/2758) will be
//       removed, so this could be `<T: Into<String>> From<T> for Error`
impl From<&str> for Error
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(message: &str) -> Self
    {
        Self::SimpleStringError(message.into())
    }
}
