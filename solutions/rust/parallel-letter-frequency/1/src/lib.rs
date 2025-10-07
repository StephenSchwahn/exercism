#![feature(hash_raw_entry)]

use std::{collections::HashMap, sync::mpsc, thread};

pub fn frequency<'a>(input: &'a [&str], worker_count: usize) -> HashMap<char, usize> {
    let (tx, rx) = mpsc::channel();

    if input.len() == 0 {
        return HashMap::new();
    }

    let inputs_per_thread = usize::max(1, usize::div_ceil(input.len(), worker_count));
    let groups: Vec<Vec<String>> = input.chunks(inputs_per_thread)
        .map(|lines| {
            lines
                .iter()
                .map(|line| String::from(*line))
                .collect::<Vec<String>>()
        })
        .collect();

    for group in groups {
        let cloned_tx = tx.clone();
        thread::spawn(move || {
            let mut inner_map: HashMap<char, usize> = HashMap::new();
            for line in group {
                for c in line.chars() {
                    if c.is_alphabetic() {
                        inner_map
                            .entry(c.to_lowercase().last().unwrap())
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                }
            }
            cloned_tx.send(
                inner_map
                    .keys()
                    .map(|key| (*key, *inner_map.get(key).unwrap_or(&0)))
                    .collect::<Vec<(char, usize)>>(),
            )
        });
    }

    // Since we used only cloned values, need to drop tx here to allow the receiver to stop
    drop(tx);

    let mut letter_frequency: HashMap<char, usize> = HashMap::new();
    for submapping in rx {
        for (key, value) in submapping {
            letter_frequency
                .entry(key)
                .and_modify(|v| *v += value)
                .or_insert(value);
        }
    }
    letter_frequency
}
