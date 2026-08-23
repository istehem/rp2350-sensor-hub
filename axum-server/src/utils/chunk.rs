use std::vec::{IntoIter, Vec};

pub fn split_into_n_chunks<T>(slice: &[T], n: usize) -> IntoIter<&[T]> {
    if n == 0 {
        return Vec::new().into_iter();
    }
    let length = slice.len();
    let base_size = length / n;
    let remainder = length % n;

    let mut result = Vec::with_capacity(n);
    let mut start = 0;
    for i in 0..n {
        let size = base_size + if i < remainder { 1 } else { 0 };
        let end = start + size;
        result.push(&slice[start..end]);
        start = end;
    }
    result.into_iter()
}
