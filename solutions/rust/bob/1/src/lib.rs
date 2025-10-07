pub fn reply(message: &str) -> &str {
    let is_caps = message
        .chars()
        .filter(|&c| c.is_alphabetic())
        .map(|c| c.is_uppercase())
        .reduce(|acc, c| acc && c);
    let is_question = message.trim().ends_with("?");
    let is_empty = message.trim().is_empty();
    
    match (is_caps, is_question, is_empty) {
        (Some(true), true, _) => "Calm down, I know what I'm doing!",
        (_, true, _) => "Sure.",
        (Some(true), false, _) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
