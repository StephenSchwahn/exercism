pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0;
    let mut n = display_value;
    while n > 0 {
        n = n & (n - 1);
        count += 1;
    }
    count
}

