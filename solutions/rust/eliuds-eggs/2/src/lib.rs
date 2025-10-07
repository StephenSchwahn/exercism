pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0usize;
    let mut n = display_value as usize;
    while n > 0 {
        count += n & 1; // If rightmost bit is 1, count it!
        n = n >> 1; // Shift the bits over to the right.
    }
    count
}

