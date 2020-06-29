use std::iter;
use std::vec;

pub struct Bytes {
    raw_data: vec::Vec<u8>,
}

pub struct BytesSplit<'a, 'b> {
    bytes: &'a Bytes,
    search_terms: &'b [u8],
    index: usize,
    finished: bool,
}

impl Bytes {
    pub fn new(data: vec::Vec<u8>) -> Self {
        return Bytes { raw_data: data };
    }

    // Returns the index to the first occurrence of search
    pub fn find(&self, start_index: usize, search_terms: &[u8]) -> Result<usize, ()> {
        for i in start_index..self.raw_data.len() {
            let index = i;

            for j in 0..search_terms.len() {
                if i > self.raw_data.len() {
                    break;
                }

                if search_terms[j] != self.raw_data[i + j] {
                    break;
                }

                if j == (search_terms.len() - 1) {
                    return Ok(index);
                }
            }
        }

        return Err(());
    }

    pub fn split<'a, 'b>(&'a self, search_terms: &'b [u8]) -> BytesSplit<'a, 'b> {
        return BytesSplit::new(&self, search_terms);
    }

    pub fn view(&self, begin: Option<usize>, end: Option<usize>) -> Result<&[u8], ()> {
        let begin_value = begin.unwrap_or(0);
        let end_value = end.unwrap_or(self.raw_data.len());

        if begin_value >= self.raw_data.len() || end_value > self.raw_data.len() {
            return Err(());
        }

        return Ok(&self.raw_data[begin_value..end_value]);
    }
}

impl<'a, 'b> BytesSplit<'a, 'b> {
    pub fn new(bytes: &'a Bytes, search_terms: &'b [u8]) -> BytesSplit<'a, 'b> {
        return BytesSplit {
            bytes: bytes,
            search_terms: search_terms,
            index: 0,
            finished: false,
        };
    }
}

impl<'a, 'b> iter::FusedIterator for BytesSplit<'a, 'b> {}

impl<'a, 'b> Iterator for BytesSplit<'a, 'b> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        match self.bytes.find(self.index, self.search_terms) {
            Ok(start_index) => {
                let previous_index = self.index;
                self.index = start_index + self.search_terms.len();
                return Some(
                    self.bytes
                        .view(Some(previous_index), Some(start_index))
                        .unwrap(),
                );
            }
            Err(_) => {
                self.finished = true;

                let view = self.bytes.view(Some(self.index), None).unwrap();
                self.index = view.len();
                return Some(view);
            }
        }
    }
}
