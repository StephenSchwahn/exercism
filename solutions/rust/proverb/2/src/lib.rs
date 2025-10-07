use std::iter::once;

use itertools::Itertools;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .join("\n"),
    }
}
