use std::fmt::{
    self,
    Display,
    Formatter,
};


/*----------------------------------------------------------------------------*/
pub enum Value
{
    Usize(usize),
}


/*----------------------------------------------------------------------------*/
impl Display for Value
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        use Value::*;

        match self
        {
            Usize(value) => write!(f, "{}", value),
        }
    }
}
