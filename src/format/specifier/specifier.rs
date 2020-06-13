use std::convert::TryInto;

use super::alignment::Alignment;
use super::actual::Actual;


/*----------------------------------------------------------------------------*/
pub struct Specifier
{
    fill: char,
    alignment: Alignment,
    width: usize,
}


/*----------------------------------------------------------------------------*/
impl Specifier
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub(super) fn fill(&self) -> char
    {
        self.fill
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub(super) fn width(&self) -> usize
    {
        self.width
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub(super) fn alignment(&self) -> Alignment
    {
        self.alignment
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn new(specifier: &str) -> crate::Result<Self>
    {
        fn error() -> crate::Error { "Incomplete specifier".into() };

        let mut indices_and_characters = specifier.char_indices();
        let (i, first) = indices_and_characters.next().ok_or_else(error)?;
        let (i, fill, alignment) =
            match first
            {
                '<' | '^' | '>' => (i, ' ', first.try_into()?),
                fill =>
                {
                    let (i, alignment) =
                        indices_and_characters.next().ok_or_else(error)?;
                    (i, fill, alignment.try_into()?)
                },
            };

        Ok(
            Self
            {
                fill,
                alignment,
                width: (&specifier[i + 1..]).parse()?,
            })
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn to_actual<'a, T>(&'a self, value: &'a T) -> Actual<'a, T>
    {
        Actual::new(self, value)
    }
}
