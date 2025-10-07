/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .try_fold(0, |sum, (i, c)| {
            c.to_digit(10)
                .map(|num| if i % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| sum + num)
        })
        .map_or(false, |sum| {
            sum % 10 == 0 && 
            code
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .len() > 1
        })
}
