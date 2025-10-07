use std::collections::HashMap;

const PAIR_MAPPING: [(char, &'static str); 4] = [
    ('V', "violets"),
    ('R', "radishes"),
    ('C', "clover"),
    ('G', "grass"),
];

// While you could use ASCII arithmetic to get this, it's not guaranteed that in the future,
// a student may graduate and leave a "hole" in the class, so I'm using a hashmap instead to
// ensure student counts are consistent
const CHILDREN_MAPPING: [(&'static str, usize); 12] = [
    ("Alice", 0),
    ("Bob", 1),
    ("Charlie", 2),
    ("David", 3),
    ("Eve", 4),
    ("Fred", 5),
    ("Ginny", 6),
    ("Harriet", 7),
    ("Ileana", 8),
    ("Joseph", 9),
    ("Kincaid", 10),
    ("Larry", 11),
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let plant_mapping: HashMap<char, &'static str> = HashMap::from(PAIR_MAPPING);
    let student_mappings: HashMap<&str, usize> = HashMap::from(CHILDREN_MAPPING);

    let rows = diagram.split_once("\n").unwrap();
    let index = *student_mappings.get(student).unwrap_or(&0);
    vec![
        rows.0.chars().nth(index*2).unwrap(),
        rows.0.chars().nth(index*2+1).unwrap(),
        rows.1.chars().nth(index*2).unwrap(),
        rows.1.chars().nth(index*2+1).unwrap(),
    ]
        .iter()
        .map(|char_at_index| plant_mapping.get(char_at_index).unwrap().to_owned())
        .collect::<Vec<&'static str>>()
}
