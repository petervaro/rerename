use std::hint::unreachable_unchecked;

use regex::{
    Captures,
    Match,
};


/*----------------------------------------------------------------------------*/
enum Reference<'a>
{
    Index(u8),
    Group(&'a str),
}


/*----------------------------------------------------------------------------*/
pub struct Referencer<'a>
{
    reference: Reference<'a>,
    getter: for<'f> unsafe fn(&Self, Captures<'f>) -> Option<Match<'f>>,
}


/*----------------------------------------------------------------------------*/
impl<'a> Referencer<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    unsafe fn by_index<'f>(&self, captures: Captures<'f>) -> Option<Match<'f>>
    {
        let index =
            match self.reference
            {
                Reference::Index(index) => index,
                _ => unreachable_unchecked(),
            };

        captures.get(index as usize)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    unsafe fn by_group<'f>(&self, captures: Captures<'f>) -> Option<Match<'f>>
    {
        let group =
            match self.reference
            {
                Reference::Group(group) => group,
                _ => unreachable_unchecked(),
            };

        captures.name(group)
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    pub fn get<'f>(&self, captures: Captures<'f>) -> Option<Match<'f>>
    {
        unsafe
        {
            (self.getter)(self, captures)
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<u8> for Referencer<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(index: u8) -> Self
    {
        Self
        {
            reference: Reference::Index(index),
            getter: Self::by_index,
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<&'a str> for Referencer<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(group: &'a str) -> Self
    {
        Self
        {
            reference: Reference::Group(group),
            getter: Self::by_group,
        }
    }
}
