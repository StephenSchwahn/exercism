use std::cmp::Ordering;

fn find_recur<T: Ord>(array: &[(T, usize)], key: T) -> Option<usize> {
    // Base case - item not in array
    if array.is_empty() {
        return None
    }

    let middle = array.len() / 2;
    match array[middle].0.cmp(&key) {
        Ordering::Equal => Some(array[middle].1),
        Ordering::Greater => find_recur(&array[..middle], key),
        Ordering::Less => find_recur(&array[middle + 1..], key)
    }
}

pub fn find<U, T>(array: U, key: T) -> Option<usize>
where
    U: AsRef<[T]>,
    T: Ord + Copy
{
    let enumerated: Vec<(&T, usize)> = array
        .as_ref()
        .iter()
        .enumerate()
        .map(|(i, val)| (val, i))
        .collect();
    find_recur(&enumerated, &key)
}
