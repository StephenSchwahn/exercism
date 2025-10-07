/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

static M: i32 = 26;
static LOWEST_ASCII_CHAR: i32 = 'a' as i32;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn mmi(a: i32) -> i32 {
    let mut x = 0;
    while (a * x).rem_euclid(M) != 1 {
        x += 1;
    }
    x
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        Ok(plaintext
            .chars()
            .map(|c: char| match c {
                c if c.is_ascii_alphabetic() => {
                    let i = c.to_ascii_lowercase() as i32 - LOWEST_ASCII_CHAR;
                    let e = ((a * i) + b).rem_euclid(M);
                    let ascii_char = (e + LOWEST_ASCII_CHAR) as u32;
                    char::from_u32(ascii_char)
                }
                c if c.is_numeric() => Some(c),
                _ => None,
            })
            .flatten()
            .collect::<Vec<char>>()
            .chunks(5)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(" "))
    }
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 {
        Err(AffineCipherError::NotCoprime(a))
    } else {
        let mmi = mmi(a);
        Ok(ciphertext
            .chars()
            .map(|c| match c {
                c if c.is_ascii_alphabetic() => {
                    let y = c.to_ascii_lowercase() as i32 - LOWEST_ASCII_CHAR;
                    let d = (mmi * (y - b)).rem_euclid(M);
                    let ascii_char = (d + LOWEST_ASCII_CHAR) as u32;
                    char::from_u32(ascii_char)
                }
                c if c.is_numeric() => Some(c),
                _ => None,
            })
            .filter_map(|o| o)
            .collect::<String>())
    }
}
