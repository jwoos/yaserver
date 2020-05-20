use crate::data::findable;
use std::iter;

pub struct TermSplit<'a, T>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    data: T,
    search_terms: &'a [T::Item],
    index: usize,
    finished: bool,
}

pub trait TermSplittable<T>: findable::Findable<T::Item>
where
    T: IntoIterator,
    T::Item: PartialEq,
{
    fn split_term(&self) -> TermSplit<T>;
}
