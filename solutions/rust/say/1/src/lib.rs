const ONES: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const TENS: [&'static str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALE_WORDS: [&'static str; 8] = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion", "sextillion"];
pub fn encode(n: u64) -> String {

    let mut thousand_groupings: Vec<u64> = Vec::new();
    let mut n_copy = n;
    while n_copy > 0 {
        thousand_groupings.insert(0, n_copy % 1000);
        n_copy /= 1000;
    }
    
    let applicable_scale_words = SCALE_WORDS[0..thousand_groupings.len()].to_vec();
    let thousand_groupings = thousand_groupings
        .iter()
        .zip(applicable_scale_words.iter().rev().map(|&scale_word| scale_word.to_owned()))
        .filter(|(&grouping, _scaling)| grouping > 0)
        .map(|(grouping, scaling)| format!("{} {}", encode_hundred(*grouping as u16), scaling).trim().to_owned())
        .collect::<Vec<String>>();

    match thousand_groupings.len() {
        0 => "zero".to_owned(),
        _ => thousand_groupings.join(" ").trim().to_owned()
    }
}

fn encode_hundred(n: u16) -> String {
    match n {
        0..=9 => ONES[n as usize].to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        15 => "fifteen".to_owned(),
        18 => "eighteen".to_owned(),
        13..=19 => format!("{}teen", encode_hundred(n % 10)),
        n if n % 10 == 0 && n < 100 => TENS[(n / 10) as usize].to_owned(),
        n if n / 100 == 0 && n < 1000 => format!("{}-{}", TENS[(n / 10) as usize], encode_hundred(n % 10)),
        n if n < 1000 => format!(
            "{} hundred {}",
            ONES[(n / 100) as usize],
            if n % 100 == 0 {
                "".to_owned()
            } else {
                encode_hundred(n % 100)
            }
        ).trim().to_owned(),
        _ => panic!("Number too large")
    }
}