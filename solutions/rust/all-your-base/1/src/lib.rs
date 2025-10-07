#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // check for invariants
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }

    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if let Some(num) = number.into_iter().find(|&num| *num >= from_base)  {
        return Err(Error::InvalidDigit(*num))
    }

    // Step-01
    // Conversion of the number to base 10 from base m with the help of the expansion method.
    let mut base_10: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, elem)| ((*elem as i64) * (from_base as i64).pow(i as u32)) as u32)
        .sum::<u32>();

    // Step-02
    // Conversion of the number to base n from base 10 with the help of the division and the multiplication method.
    let mut base_vec = Vec::new();
    loop {
        let remainder = base_10 % to_base;
        base_vec.push(remainder);

        base_10 = base_10 / to_base;
        if base_10 == 0 {
            break
        }
    }
    base_vec.reverse();
    Ok(base_vec)
}
