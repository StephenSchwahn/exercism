/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .len()
        < 2
        || code.chars().any(|c| !(c.is_ascii_digit() || c.is_whitespace()))
    {
        return false;
    } else {
        code.chars()
            .filter(|c| c.is_numeric())
            .map(|c| {
                u8::from_str_radix(&format!("{}", c), 10).expect("char must be number from 0-9")
            })
            .rev()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 1 {
                    let doubled = c * 2;
                    if doubled > 9 {
                        doubled - 9
                    } else {
                        doubled
                    }
                } else {
                    c
                }
            })
            .fold(0u32, |acc, num| acc + num as u32)
            % 10
            == 0
    }
}
