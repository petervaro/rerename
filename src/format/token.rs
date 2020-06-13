use super::variable::Variable;


/*----------------------------------------------------------------------------*/
pub enum Token<'a>
{
    Value(&'a str),
    Variable(Variable<'a>),
}
