pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::with_capacity(string.len());
    for c in string.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        }
        if c == ')' || c == ']' || c == '}' {
            let popped = stack.pop();
            if popped.is_none() {
                return false;
            }
            match (popped.unwrap(), c) {
                ('(', ')') => true,
                ('[', ']') => true,
                ('{', '}') => true,
                _ => return false
            };
        }
    } 
    if stack.len() != 0 { false } else { true }
}
