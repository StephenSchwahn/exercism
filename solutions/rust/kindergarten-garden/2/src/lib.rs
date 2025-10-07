const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_plant_idx = CHILDREN
        .iter()
        .position(|&s| student == s)
        .unwrap_or_default();

    diagram
        .lines()
        .flat_map(|line| {
            line[student_plant_idx..=student_plant_idx + 1]
                .chars()
                .map(|plant| match plant {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    _ => "violets",
                })
        })
        .collect()
}
