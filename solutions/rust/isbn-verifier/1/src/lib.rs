/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = isbn
        .chars()
        .filter(|c| c.is_digit(10) || *c == 'X')
        .collect();
    
    let x_count = isbn.chars().filter(|c|  *c == 'X').count();
    if isbn.len() != 10 || x_count > 1 || x_count == 1 && !isbn.ends_with("X") {
        return false;
    }

    isbn
        .char_indices()
        .map(|(index, c)| {
            match c {
                'X' => 10,
                _ => c.to_digit(10).unwrap() * (10 - index as u32)
            }
        })
        .sum::<u32>() % 11 == 0
}
