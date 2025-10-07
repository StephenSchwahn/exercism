fn num_to_string_with_capitalization(num: u32, capitalize_first_letter: bool) -> String {
    const NUM_STRS: [&str; 11] = 
    ["no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    
    match capitalize_first_letter {
        false => NUM_STRS[num as usize].to_string(),
        true => { 
            let mut chars = NUM_STRS[num as usize].chars();
            let first = chars.next().unwrap(); // All these known strings have > 0 length
            first.to_uppercase().chain(chars).collect()
        }
    }
}

fn verse(n: u32) -> String {
    format!("{num_str} green bottle{plural} hanging on the wall,\n{num_str} green bottle{plural} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {num_less_str} green bottle{plural_num_less} hanging on the wall.",
            num_str = num_to_string_with_capitalization(n, true),
            plural = if n == 1 { "" } else { "s" }, 
            num_less_str = num_to_string_with_capitalization(n-1, false),
            plural_num_less = if (n-1) == 1 { "" } else { "s" }
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if start_bottles > 10 {
        panic!("Only supports start_value up to 10");
    }
    ((start_bottles-take_down+1)..=start_bottles)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n\n")
}