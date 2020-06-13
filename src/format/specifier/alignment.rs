use std::convert::TryFrom;


/*----------------------------------------------------------------------------*/
#[derive(Clone, Copy)]
pub enum Alignment
{
    Left,
    Centre,
    Right,
}


/*----------------------------------------------------------------------------*/
impl TryFrom<char> for Alignment
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    type Error = crate::Error;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn try_from(alignment: char) -> Result<Self, Self::Error>
    {
        use Alignment::*;

        match alignment
        {
            '<' => Ok(Left),
            '^' => Ok(Centre),
            '>' => Ok(Right),
            invalid => Err(format!("Invalid alignment: `{}`", invalid).into()),
        }
    }
}
