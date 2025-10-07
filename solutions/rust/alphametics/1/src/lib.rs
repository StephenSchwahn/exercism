use itertools::Itertools;
use std::collections::HashMap;

fn convert_str_to_num(str_num: &str, mapping: &HashMap<char, u8>) -> u64 {
    let num = str_num
        .chars()
        .map(|letter| *mapping.get(&letter).expect("inconceivable"))
        .rev()
        .enumerate()
        .fold(0u64, |acc, (i, num)| {
            acc + (num as u64) * 10u64.pow(i as u32)
        });
    num
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut keys: Vec<char> = Vec::new();
    for letter in input.chars() {
        if letter.is_alphabetic() && !keys.contains(&letter) {
            keys.push(letter);
        }
    }

    let (addends, sum) = input.split_once(" == ").expect("Invalid Equation");
    let addends: Vec<&str> = addends.split(" + ").collect();

    for perm in (0..=9).permutations(keys.len()) {
        let mapping: HashMap<char, u8> = perm
            .iter()
            .zip(keys.iter())
            .map(|(&value, &key)| (key, value))
            .collect();

        let sum_with_leading_zero = convert_str_to_num(&sum[..1], &mapping) == 0;
        let an_addend_has_leading_zero = addends
                .iter()
                .any(|addend| convert_str_to_num(&addend[..1], &mapping) == 0);
        if sum_with_leading_zero || an_addend_has_leading_zero {
            continue;
        }

        let numeric_addend_sum: u64 = addends
            .iter()
            .map(|&str_num| convert_str_to_num(str_num, &mapping))
            .sum();

        let numeric_sum = convert_str_to_num(sum, &mapping);

        if numeric_addend_sum == numeric_sum {
            return Some(mapping);
        }
    }
    None
}
