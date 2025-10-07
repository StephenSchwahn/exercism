pub fn encrypt(input: &str) -> String {
    let filtered = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();

    let c = (filtered.len() as f64).sqrt().ceil() as usize;
    let matrix = filtered
        .chunks(if c == 0 { 1 } else { c })
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<char>>>();

    (0..c)
        .map(|i| {
            matrix
                .iter()
                .map(|inner| if i < inner.len() { inner[i] } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
