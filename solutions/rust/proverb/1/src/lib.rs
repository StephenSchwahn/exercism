#![feature(iter_map_windows)]
use itertools::Itertools;

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => "".to_owned(),
        1 => format!("And all for the want of a {}.", list[0]),
        _ => {
            let last = format!("And all for the want of a {}.", list[0]);
            list.iter()
                .map_windows(|[x, y]| format!("For want of a {} the {} was lost.", x, y))
                .chain([last])
                .join("\n") 
            }
    }
}
