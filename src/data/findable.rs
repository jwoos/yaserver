use std::iter;
use std::vec;

pub trait Findable<T>: IntoIterator
where
    T: PartialEq,
{
    fn find_term(&self, start_index: usize, search_terms: &[T]) -> Option<usize>;
}

impl<T> Findable<T> for &[T]
where
    T: PartialEq,
{
    fn find_term(&self, start_index: usize, search_terms: &[T]) -> Option<usize> {
        let mut start_index = 0;
        let mut index = 0;
        for (i, c) in self.into_iter().enumerate() {
            let mut reset = false;
            loop {
                if search_terms[index] == *c {
                    if index == 0 {
                        start_index = i;
                    }
                    index += 1;
                    break;
                } else if !reset {
                    index = 0;
                    reset = true;
                } else {
                    index = 0;
                    break;
                }
            }

            if index > (search_terms.len() - 1) {
                return Some(start_index);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::data::findable::Findable;

    const TEST_DATA: &[u8] = &[0, 0, 0, 1, 2, 3, 0, 0];

    #[test]
    fn not_found() {
        let found = TEST_DATA.find_term(0, &[5]);
        assert_eq!(None, found);
    }

    #[test]
    fn found_single() {
        let found = TEST_DATA.find_term(0, &[0]);
        assert_ne!(None, found);
        assert_eq!(0, found.unwrap());

        let found = TEST_DATA.find_term(0, &[1]);
        assert_ne!(None, found);
        assert_eq!(3, found.unwrap());

        let found = TEST_DATA.find_term(0, &[2]);
        assert_ne!(None, found);
        assert_eq!(4, found.unwrap());
    }

    #[test]
    fn found_multiple() {
        let x: [u8; 8] = [0, 0, 0, 1, 2, 3, 0, 0];
        let found = (&x[..]).find_term(0, &[1, 2]);
        assert_ne!(None, found);
        assert_eq!(3, found.unwrap());
    }
}
