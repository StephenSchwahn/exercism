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
        let str_val = value.to_string();
        let (mut x, mut y) = (0, str_val.len() - 1);
        while x < y {
            if str_val[x..x + 1] != str_val[y..y + 1] {
                return None;
            }
            x += 1;
            y -= 1;
        }
        Some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let pals: BinaryHeap<Palindrome> = (min..=max)
        .combinations_with_replacement(2)
        .map(|combinations| combinations[0] * combinations[1])
        .unique()
        .map(Palindrome::new)
        .filter(|pal| pal.is_some())
        .map(|pal| pal.unwrap())
        .collect();

    let pals = pals.into_sorted_vec();
    if pals.len() > 0 {
        Some((pals[0], pals[pals.len() - 1]))
    } else {
        None
    }
}
