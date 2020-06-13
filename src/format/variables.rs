use std::ops::Deref;

use super::value::Value;


/*----------------------------------------------------------------------------*/
pub struct Variables
{
    index: usize,
}


/*----------------------------------------------------------------------------*/
impl Variables
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(index: usize) -> Self
    {
        Self { index }
    }

    pub fn names(&self) -> impl Iterator<Item = &str>
    {
        ["index"].iter().map(Deref::deref)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn get(&self, variable: &str) -> Option<Value>
    {
        match variable
        {
            "index" => Some(Value::Usize(self.index)),
            _ => None,
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn index_mut(&mut self) -> &mut usize
    {
        &mut self.index
    }
}
