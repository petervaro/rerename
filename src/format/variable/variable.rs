use crate::format::specifier::Specifier;
use super::actual::Actual;


/*----------------------------------------------------------------------------*/
pub struct Variable<'a>
{
    name: &'a str,
    specifier: Option<Specifier>,
}


/*----------------------------------------------------------------------------*/
impl<'a> Variable<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub(super) fn specifier(&self) -> Option<&Specifier>
    {
        self.specifier.as_ref()
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(name: &'a str,
               specifier: Option<&'a str>) -> crate::Result<Self>
    {
        let specifier =
            match specifier
            {
                Some(specifier) => Some(Specifier::new(specifier)?),
                _ => None,
            };

        Ok(
            Self
            {
                name,
                specifier,
            })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn name(&self) -> &'_ str
    {
        &self.name
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn to_actual<'f, T>(&'f self, value: &'f T) -> Actual<'f, T>
    {
        Actual::new(self, value)
    }
}
