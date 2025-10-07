use itertools::Itertools;
use std::collections::BinaryHeap;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if value % 10 == 0 {
            return None;
        };
        let mut reverse = 0;
        let mut r = value;
        while r > 0 {
            reverse = reverse * 10 + r % 10;
            r /= 10;
        }
        
        if value == reverse {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_p: Option<Palindrome> = None;
    let mut max_p: Option<Palindrome> = None;

    for x in min..=max {
        if x % 10 == 0 {
            continue;
        };
        for y in min..=max {
            if y % 10 == 0 {
                continue;
            };
            if let Some(pal) = Palindrome::new(x * y) {
                if pal.0 < min_p.map(|pal| pal.0).unwrap_or(u64::MAX) {
                    min_p = Some(pal);
                }
                if pal.0 > max_p.map(|pal| pal.0).unwrap_or(u64::MIN) {
                    max_p = Some(pal);
                }
            }
        }
    }

    match (min_p, max_p) {
        (Some(min_pal), Some(max_pal)) => Some((min_pal, max_pal)),
        _ => None,
    }
}
