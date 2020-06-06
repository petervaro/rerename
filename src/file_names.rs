use std::{
    ops::Deref,
    slice::Iter,
    hint::unreachable_unchecked,
};

use clap::Values;


/*----------------------------------------------------------------------------*/
enum FileNamesIter<'a>
{
    ArgValues(Values<'a>),
    SliceIter(Iter<'a, &'a str>),
}


/*----------------------------------------------------------------------------*/
pub struct FileNames<'a>
{
    inner: FileNamesIter<'a>,
    stepper: unsafe fn(&mut Self) -> Option<<Self as Iterator>::Item>
}


/*----------------------------------------------------------------------------*/
impl<'a> FileNames<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    unsafe fn next_from_arg_values(&mut self) -> Option<<Self as Iterator>::Item>
    {
        match &mut self.inner
        {
            FileNamesIter::ArgValues(inner) => inner.next(),
            _ => unreachable_unchecked(),
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    unsafe fn next_from_slice_iter(&mut self) -> Option<<Self as Iterator>::Item>
    {
        match &mut self.inner
        {
            FileNamesIter::SliceIter(inner) => inner.next().map(Deref::deref),
            _ => unreachable_unchecked(),
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<Values<'a>> for FileNames<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(values: Values<'a>) -> Self
    {
        Self
        {
            inner: FileNamesIter::ArgValues(values),
            stepper: Self::next_from_arg_values,
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> From<Iter<'a, &'a str>> for FileNames<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn from(iter: Iter<'a, &'a str>) -> Self
    {
        Self
        {
            inner: FileNamesIter::SliceIter(iter),
            stepper: Self::next_from_slice_iter,
        }
    }
}


/*----------------------------------------------------------------------------*/
impl<'a> Iterator for FileNames<'a>
{
    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    type Item = &'a str;

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn next(&mut self) -> Option<Self::Item>
    {
        unsafe
        {
            (self.stepper)(self)
        }
    }
}
