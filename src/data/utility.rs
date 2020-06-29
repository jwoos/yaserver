pub fn find(data: &[u8], start_index: usize, search_terms: &[u8]) -> Option<usize> {
    let mut search_start_index = 0;
    let mut index = 0;
    for (i, c) in data.into_iter().enumerate() {
        if i < start_index {
            continue;
        }

        let mut reset = false;
        loop {
            if search_terms[index] == *c {
                if index == 0 {
                    search_start_index = i;
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
            return Some(search_start_index);
        }
    }

    return None;
}
