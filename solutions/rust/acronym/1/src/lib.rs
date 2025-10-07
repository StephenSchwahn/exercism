#![feature(iter_map_windows)]

pub fn abbreviate(phrase: &str) -> String {
    format!(" {}", phrase) // Add a space at the start to ensure first char is in the acronym
        .replace("-", " ") // Treat hyphens as spaces
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace()) 
        .map_windows(|[c1, c2]| {
            if (c1.is_whitespace()) || (c1.is_lowercase() && c2.is_uppercase()) {
                *c2
            } else {
                ' '
            }
        })
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_uppercase())
        .collect::<String>()
}
