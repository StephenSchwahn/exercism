use itertools::Itertools;
use std::collections::HashMap;

fn convert_str_to_num(str_num: &str, mapping: &HashMap<&char, u8>) -> u64 {
    str_num
        .trim()
        .chars()
        .rev()
        .enumerate()
        .map(|(i, letter)| {
            *mapping.get(&letter).expect("inconceivable") as u64 * 10_u64.pow(i as u32)
        })
        .sum()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (addends, sum) = input.split_once(" == ").expect("Invalid Equation");
    let addends: Vec<&str> = addends.split(" + ").collect();

    let letters: Vec<char> = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .unique()
        .collect();

    let leading_letters: Vec<char> = input
        .split(['+', '='])
        .filter(|s| s.trim().len() > 1)
        .map(|s| s.trim().chars().nth(0).unwrap())
        .unique()
        .collect();

    for mapping in (0..=9)
        .permutations(letters.len())
        .map(|perm| {
            letters
                .iter()
                .sorted()
                .zip(perm)
                .collect::<HashMap<&char, u8>>()
        })
        .filter(|mapping| leading_letters.iter().all(|c| *mapping.get(c).unwrap() != 0))
    {
        let numeric_addend_sum: u64 = addends
            .iter()
            .map(|&str_num| convert_str_to_num(str_num, &mapping))
            .sum();

        let numeric_sum = convert_str_to_num(sum, &mapping);

        if numeric_addend_sum == numeric_sum {
            return Some(mapping.iter().map(|(&&c, &i)| (c, i)).collect());
        }
    }
    None
}
