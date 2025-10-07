fn find_recur<T: Ord + Eq>(array: &[(T, usize)], key: T) -> Option<usize> {
    let middle = array.len() / 2;

    // Base case - item not in array
    if array.len() == 0 {
        None
    }
    // Base Case - item is in the middle
    else if array[middle].0 == key {
        Some(array[middle].1)
    }
    else if array[middle].0 > key {
        // Discard right half
        find_recur(&array[..middle], key)
    } else {
        // Discard left half
        find_recur(&array[middle+1..], key)
    }
}

pub fn find<T: Ord + Eq + Copy>(array: &[T], key: T) -> Option<usize> {
    let enumerated: Vec<(T, usize)> = array.iter().enumerate().map(|(i, &val)| { (val, i)}).collect();
    find_recur(&enumerated.as_slice(), key)
}