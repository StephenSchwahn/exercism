pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => vec![String::new(); len],
        _ => digits
            .chars()
            .collect::<Vec<char>>()
            .windows(len)
            .map(|chars| chars.iter().collect())
            .collect::<Vec<String>>(),
    }
}
