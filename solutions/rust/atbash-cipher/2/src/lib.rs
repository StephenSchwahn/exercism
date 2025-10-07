fn get_reversed_character(c: char) -> Option<char> {
    match c {
        '0'..='9' => Some(c),
        _ => ('a'..='z')
            .rev()
            .nth((c.to_ascii_lowercase() as u32 - 'a' as u32) as usize),
    }
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(|c| c.is_alphanumeric())
        .filter_map(get_reversed_character)
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_alphanumeric())
        .filter_map(get_reversed_character)
        .collect::<String>()
}
