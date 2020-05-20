use std::iter;

pub trait Findable<T>
where
    T: Iterator,
{
    fn find(&self, start_index: usize, search_terms: &[T::Item]) -> Option<usize>;
}
