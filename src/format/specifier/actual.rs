use std::fmt::{
    self,
    Display,
    Formatter,
};

use super::{
    Specifier,
    alignment::Alignment,
};


/*----------------------------------------------------------------------------*/
pub struct Actual<'a, T>
{
    specifier: &'a Specifier,
    value: &'a T,
}


/*----------------------------------------------------------------------------*/
impl<'a, T> Actual<'a, T>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(specifier: &'a Specifier,
               value: &'a T) -> Self
    {
        Self { specifier, value }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a, T> Display for Actual<'a, T>
    where T: Display
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        use Alignment::*;

        let formatted = format!("{}", self.value);
        let remaining =
            {
                let width = self.specifier.width();
                let taken = formatted.chars().count();
                if width > taken { width - taken } else { 0 }
            };
        let fill = self.specifier.fill();

        match self.specifier.alignment()
        {
            Left =>
            {
                write!(f, "{}", formatted)?;
                for _ in 0..remaining { write!(f, "{}", fill)?; }
                Ok(())
            },
            Centre =>
            {
                let left = remaining/2;
                let right = left + remaining%2;
                for _ in 0..left { write!(f, "{}", fill)?; }
                write!(f, "{}", formatted)?;
                for _ in 0..right { write!(f, "{}", fill)?; }
                Ok(())
            },
            Right =>
            {
                for _ in 0..remaining { write!(f, "{}", fill)?; }
                write!(f, "{}", formatted)
            },
        }
    }
}
