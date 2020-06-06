use crate::comparable::Comparable;


/*----------------------------------------------------------------------------*/
pub struct Converter
{
    converter: for<'a> fn(&'a str) -> crate::Result<Comparable<'a>>,
}


/*----------------------------------------------------------------------------*/
impl Converter
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn to_comparable_integer<'a>(value: &'a str) -> crate::Result<Comparable<'a>>
    {
        Ok(value.parse::<usize>()?.into())
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn to_comparable_string<'a>(value: &'a str) -> crate::Result<Comparable<'a>>
    {
        Ok(value.into())
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(kind: &str) -> crate::Result<Self>
    {
        let converter =
            match kind
            {
                "int" => Self { converter: Self::to_comparable_integer },
                "str" => Self { converter: Self::to_comparable_string },
                _ =>
                {
                    let message = format!("Invalid `type`: {}", kind);
                    Err(crate::Error::SimpleStringError(message))?
                },
            };

        Ok(converter)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn to_comparable<'a>(&self, value: &'a str) -> crate::Result<Comparable<'a>>
    {
        (self.converter)(value)
    }
}
