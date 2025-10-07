pub fn encode(source: &str) -> String {
    let source = format!("{}\0", source);
    source
        .chars()
        .fold(
            ('\0', 0, String::new()),
            |(prev_char, prev_count, acc), curr: char| {
                if curr == prev_char {
                    (prev_char, prev_count + 1, acc)
                } else {
                    match prev_count {
                        0 | 1 => (curr, 1, format!("{}{}", acc, prev_char)),
                        _ => (curr, 1, format!("{}{}{}", acc, prev_count, prev_char)),
                    }
                }
            },
        )
        .2
        .replace("\0", "")
        .to_owned()
}

pub fn decode(source: &str) -> String {
    let mut decoded: Vec<String> = Vec::new();
    let mut digit_open = false;
    let mut digit_str = String::new();

    for ch in source.chars() {
        if ch.is_digit(10) {
            digit_open = true;
            digit_str.push(ch);
        }
        else if digit_open {
            decoded.push(format!("{}", ch).repeat(digit_str.parse().unwrap()));
            digit_open = false;
            digit_str = String::new();
        }
        else {
            decoded.push(format!("{}", ch));
        }
    }

    decoded.join("")
}
