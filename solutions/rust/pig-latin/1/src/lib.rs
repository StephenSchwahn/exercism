pub fn translate<'a>(input: &'a str) -> String {
    let vowel_like = |str: &str| match str {
        s if s.starts_with(['a', 'e', 'i', 'o', 'u']) => true,
        s if s.starts_with("xr") => true,
        s if s.starts_with("yt") => true,
        _ => false,
    };
    let consonant = |str: &'a str| -> Option<(String, String)> {
        if vowel_like(str) {
            None
        } else {
            if str.starts_with("sc")
                || str.starts_with("st")
                || str.starts_with("ch")
                || str.starts_with("rh")
                || str.starts_with("qu")
                || str.starts_with("th")
            {
                let (word_consonant, rest) = str.split_at(2);
                if vowel_like(rest) {
                    Some((word_consonant.to_owned(), rest.to_owned()))
                } else {
                    let (rest_consonant, rest_rest) = rest.split_at(1); 
                    Some((format!("{}{}", word_consonant, rest_consonant), rest_rest.to_owned()))
                }
                
            } else {
                let split = str.split_at(1);
                Some((split.0.to_owned(), split.1.to_owned()))
            }
        }
    };

    input
        .split_ascii_whitespace()
        .map(|word| {
            if vowel_like(word) {
                format!("{}ay", word)
            } else if let Some((consonant, rest)) = consonant(word) {
                if rest.starts_with("qu") {
                    // "square" -> "aresquay"
                    let (rest_start, rest_rest) = rest.split_at(2);
                    format!("{}{}{}ay", rest_rest, consonant, rest_start)
                } else if rest.starts_with("y") {
                    // "rhythm" -> "ythmrhay" or "my" -> "ymay").
                    format!("{}{}ay", rest, consonant)
                } else {
                    format!("{}{}ay", rest, consonant)
                }
            } else {
                format!("{}", word)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
