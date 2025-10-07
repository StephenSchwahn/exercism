fn calc_columns(message_len: usize) -> usize {
    let mut c = f32::sqrt(message_len as f32).floor() as usize;
    let mut r: usize = f32::sqrt(message_len as f32).floor() as usize;

    loop {
        if c * r >= message_len && c >= r && c - r <= 1 {
            return c;
        }

        if c - r >= 1 {
            r += 1;
        } else {
            c += 1;
        }
    }
}

pub fn encrypt(input: &str) -> String {
    let filtered = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<char>>();

    let c = calc_columns(filtered.len());
    let matrix = filtered
        .chunks(if c == 0 { 1 } else { c })
        .map(|c| c.to_vec())
        .collect::<Vec<Vec<char>>>();

    (0..c)
        .map(|i| {
            matrix
                .iter()
                .map(|inner| {
                    if i < inner.len() {
                        inner[i].clone()
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}
