use crate::data::term_findable;
use std::marker;
use std::vec;

pub struct TermSplit<'a, T, U>
where
    T: IntoIterator,
{
    data: &'a T,
    search_terms: &'a [U],
    index: usize,
    finished: bool,
}

pub trait TermSplittable<T>: IntoIterator + term_findable::TermFindable<T>
where
    T: PartialEq,
    <Self as std::iter::IntoIterator>::Item: PartialEq,
    Self: marker::Sized,
{
    fn split_term<'a>(&'a self, search_terms: &'a [T]) -> TermSplit<'a, Self, T> {
        return TermSplit {
            data: self,
            search_terms: search_terms,
            index: 0,
            finished: false,
        };
    }
}

impl<T> TermSplittable<T> for vec::Vec<T> where T: PartialEq {}
