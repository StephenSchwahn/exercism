use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut used_letters = HashSet::with_capacity(candidate.len());
    for c in candidate.to_ascii_lowercase().chars() {
        if !used_letters.insert(c.to_ascii_lowercase()) && c.is_alphabetic() {
            return false;
        }
    }
    true
}
