pub fn find(array: &[i32], key: i32) -> Option<usize> {
    find_rec(array, key, 0)
}

fn find_rec(array: &[i32], key: i32, start_index: usize) -> Option<usize> {
    if array.len() == 0 {
        None
    } else if array.len() == 1 {
        if array[0] == key {
            Some(start_index)
        } else {
            None
        }
    } else {
        let mid = array.len() / 2;
        if array[mid - 1] < key {
            find_rec(&array[mid..], key, start_index + mid)
        } else {
            find_rec(&array[..mid], key, start_index)
        }
    }
}
