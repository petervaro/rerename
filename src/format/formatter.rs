use std::fmt::{
    self,
    Write,
};

use super::{
    Variables,
    token::Token,
    parser::Parser,
};





/*----------------------------------------------------------------------------*/
pub struct Formatter<'a>
{
    tokens: Vec<Token<'a>>,
}


/*----------------------------------------------------------------------------*/
impl<'a> Formatter<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new<'f>(format_string: &'a str,
                   variables: &'f Variables) -> crate::Result<Self>
    {
        let mut tokens = Vec::new();
        for token in Parser::new(format_string, variables.names().collect())
        {
            tokens.push(token?);
        }

        Ok(Self { tokens })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn format<T>(&self, variables: &Variables,
                            buffer: &mut T) -> fmt::Result
        where T: Write
    {
        use Token::*;

        for token in self.tokens.iter()
        {
            match token
            {
                Value(text) => write!(buffer, "{}", text)?,
                Variable(variable) =>
                {
                    let value = variables.get(variable.name()).unwrap();
                    write!(buffer, "{}", variable.to_actual(&value))?;
                }
            }
        }

        Ok(())
    }
}
