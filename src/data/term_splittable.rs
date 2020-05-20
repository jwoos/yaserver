use crate::data::findable;
use std::marker;

pub struct TermSplit<'a, T>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    data: &'a T,
    search_terms: &'a [T::Item],
    index: usize,
    finished: bool,
}

pub trait TermSplittable: IntoIterator + findable::Findable<<Self as IntoIterator>::Item>
where
    <Self as std::iter::IntoIterator>::Item: PartialEq,
    Self: marker::Sized,
{
    fn split_term<'a>(&'a self, search_terms: &'a [Self::Item]) -> TermSplit<'a, Self> {
        return TermSplit {
            data: self,
            search_terms: search_terms,
            index: 0,
            finished: false,
        };
    }
}
