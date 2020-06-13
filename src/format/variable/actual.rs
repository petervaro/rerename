use std::fmt::{
    self,
    Display,
    Formatter,
};

use super::Variable;


/*----------------------------------------------------------------------------*/
pub struct Actual<'a, T>
{
    variable: &'a Variable<'a>,
    value: &'a T,
}


/*----------------------------------------------------------------------------*/
impl<'a, T> Actual<'a, T>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(variable: &'a Variable<'a>,
               value: &'a T) -> Self
    {
        Self { variable, value }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a, T> Display for Actual<'a, T>
    where T: Display
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        match &self.variable.specifier()
        {
            Some(specifier) =>
                write!(f, "{}", specifier.to_actual(&self.value)),
            None => write!(f, "{}", self.value),
        }
    }
}
