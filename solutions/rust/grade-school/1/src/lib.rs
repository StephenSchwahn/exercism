use itertools::Itertools;
use std::collections::{HashMap, HashSet};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    pub roster: HashMap<u32, Vec<String>>,
    pub all_students: HashSet<String>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
            all_students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.all_students.contains(student) {
            panic!("Cannot add student to more than one place in the roster")
        }
        self.all_students.insert(student.to_owned());
        self.roster
            .entry(grade)
            .and_modify(|list| list.push(student.to_owned()))
            .or_insert(vec![student.to_owned()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().sorted().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .map(|students| students.iter().sorted().cloned().collect::<Vec<String>>())
            .unwrap_or(Vec::new())
    }
}
