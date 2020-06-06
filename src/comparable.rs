use std::hint::unreachable_unchecked;

use std::cmp::Ordering;


/*----------------------------------------------------------------------------*/
enum Value<'a>
{
    Integer(usize),
    String(&'a str),
}


/*----------------------------------------------------------------------------*/
pub struct Comparable<'a>
{
    value: Value<'a>,
    comparator: unsafe fn(&Self, &Self) -> Ordering,
}


/*----------------------------------------------------------------------------*/
impl<'a> Comparable<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    unsafe fn compare_integers(&self, other: &Self) -> Ordering
    {
        use Value::*;

        match (&self.value, &other.value)
        {
            (Integer(left), Integer(right)) => left.cmp(right),
            _ => unreachable_unchecked(),
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    unsafe fn compare_strings(&self, other: &Self) -> Ordering
    {
        use Value::*;

        match (&self.value, &other.value)
        {
            (String(left), String(right)) => left.cmp(right),
            _ => unreachable_unchecked(),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<usize> for Comparable<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(integer: usize) -> Self
    {
        Self
        {
            value: Value::Integer(integer),
            comparator: Self::compare_integers,
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<&'a str> for Comparable<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(string: &'a str) -> Self
    {
        Self
        {
            value: Value::String(string),
            comparator: Self::compare_strings,
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Ord for Comparable<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn cmp(&self, other: &Self) -> Ordering
    {
        unsafe
        {
            (self.comparator)(self, other)
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> PartialOrd for Comparable<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Eq for Comparable<'a> {}


/*----------------------------------------------------------------------------*/
impl<'a> PartialEq for Comparable<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn eq(&self, other: &Self) -> bool
    {
        self.cmp(other) == Ordering::Equal
    }
}
