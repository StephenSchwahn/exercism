pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        vec![String::new(); len]
    } else {
        digits
            .as_bytes()
            .windows(len)
            .map(|chars| chars.iter().map(|c| *c as char).collect())
            .collect::<Vec<String>>()
    }
}
