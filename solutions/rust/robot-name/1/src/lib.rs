use lazy_static::lazy_static;
use rand::{distr::{Alphabetic, Uniform}, Rng};
use std::{collections::HashSet, sync::Mutex};

fn rand_name() -> String {
    rand::rng()
        .sample_iter(Alphabetic)
            .take(2)
            .map(|c| c.to_ascii_uppercase() as char)
        .chain(
            rand::rng().sample_iter(Uniform::new(0, 10).unwrap())
                .take(3)
                .map(|n| char::from_digit(n, 10).unwrap())
        )
        .collect::<String>()
}


lazy_static! {
    static ref TAKEN_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn generate_unique_random() -> String {
    let mut name = rand_name();
    while TAKEN_NAMES.lock().unwrap().contains(&name) {
        name = rand_name();
    }
    TAKEN_NAMES.lock().unwrap().insert(name.to_owned());
    name
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: generate_unique_random()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = generate_unique_random()
    }
}
