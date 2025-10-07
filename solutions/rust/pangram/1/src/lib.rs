use itertools::Itertools;

const NUM_LETTERS: usize = 26;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .unique()
        .count() == NUM_LETTERS
}
