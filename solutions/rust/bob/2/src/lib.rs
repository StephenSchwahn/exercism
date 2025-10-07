pub fn reply(message: &str) -> &str {
    let message = message.trim_end();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_caps = message.contains(char::is_alphabetic) && !message.contains(char::is_lowercase);
    let is_question = message.ends_with("?");

    match (is_caps, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (false, true) => "Sure.",
        (true, false) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
