use crate::data::findable;
use std::iter;

pub struct TermSplit<'a, T>
where
    T: Iterator,
{
    data: T,
    search_terms: &'a [T::Item],
    index: usize,
    finished: bool,
}

pub trait TermSplittable<T>: findable::Findable<T>
where
    T: Iterator,
{
    fn split(&self) -> TermSplit<T>;
}
