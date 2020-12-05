pub fn find(array: &[i32], key: i32) -> Option<usize> {    
    check_slice(0, key, array)
}

fn check_slice(cum_index: usize, key: i32, slice: &[i32]) -> Option<usize> {
    let half_index: usize = slice.len().div_euclid(2);
    let (left_chunk, right_chunk) = slice.split_at(half_index);
    if let Some(first_element) = right_chunk.first() {
        if *first_element == key {
            return Some(cum_index + left_chunk.len());
        } else if *first_element < key && right_chunk.len() > 1 {
            return check_slice(cum_index + half_index, key, right_chunk);
        } else if *first_element > key {
            return check_slice(cum_index, key, left_chunk);
        }
    }

    None
}
