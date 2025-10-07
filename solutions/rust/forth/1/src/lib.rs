use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut resolved: String = input.to_lowercase();
        // First, check if there are any word definitions and strip them out
        while let (Some(start_idx), Some(end_idx)) =
            (resolved.find(|c| c == ':'), resolved.find(|c| c == ';'))
        {
            let word_exp = resolved[start_idx + 1..end_idx].trim();
            let mut tokens: Vec<String> =
                word_exp.split_whitespace().map(|s| s.to_owned()).collect();

            if tokens.len() > 1 && tokens[0].parse::<Value>().is_err() {
                let curr_key = tokens[0].clone();

                // Pre-Expand out only defintions that will change so they have a point-in-time value
                if self.words.contains_key(&curr_key) {
                    let curr_val = self.words[&curr_key].clone();

                    // Check if any other key/value pairs have this word
                    for (_key, value) in self.words.iter_mut() {
                        if value.contains(&curr_key) {
                            let expanded_value = value.replace(&curr_key, &curr_val);
                            *value = expanded_value;
                        }
                    }

                    // Check if any tokens in the word definition have this word
                    for value in &mut tokens[1..] {
                        if value == &curr_key {
                            let expanded_value = value.replace(&curr_key, &curr_val);
                            *value = expanded_value
                        }
                    }
                }

                // Insert or update definition
                self.words
                    .insert(tokens[0].to_owned(), tokens[1..].join(" "));
            } else {
                return Err(Error::InvalidWord);
            }
            resolved.replace_range(start_idx..=end_idx, "")
        }

        // Then replace all words in the input with their resolved values
        for key in self.words.keys() {
            resolved = resolved.replace(key, self.words.get(key).unwrap())
        }

        // Finally, evalute the total string
        let tokens = resolved.split_whitespace();
        for token in tokens {
            let numeric_token = token.parse::<Value>();
            match token {
                _ if numeric_token.is_ok() => {
                    self.stack.push(numeric_token.unwrap());
                }
                "+" => self.perform_numeric_op(Value::checked_add)?,
                "-" => self.perform_numeric_op(Value::checked_sub)?,
                "*" => self.perform_numeric_op(Value::checked_mul)?,
                "/" => self.perform_numeric_op(Value::checked_div)?,
                _ if token == "dup" => {
                    if let Some(last) = self.stack.last() {
                        self.stack.push(last.clone());
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                _ if token == "drop" => {
                    if let None = self.stack.pop() {
                        return Err(Error::StackUnderflow);
                    }
                }
                _ if token == "swap" => {
                    if let (Some(n1), Some(n2)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(n1);
                        self.stack.push(n2);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                _ if token == "over" => {
                    if let (Some(n1), Some(n2)) = (self.stack.pop(), self.stack.pop()) {
                        self.stack.push(n2);
                        self.stack.push(n1);
                        self.stack.push(n2);
                    } else {
                        return Err(Error::StackUnderflow);
                    }
                }
                ":" | ";" => return Err(Error::InvalidWord),
                _ => return Err(Error::UnknownWord),
            };
        }
        Ok(())
    }

    fn perform_numeric_op(
        &mut self,
        op: fn(Value, Value) -> Option<Value>,
    ) -> std::result::Result<(), Error> {
        if self.stack.len() <= 1 {
            return Err(Error::StackUnderflow);
        }
        let n2 = self.stack.pop().unwrap();
        let n1 = self.stack.pop().unwrap();

        let result = op(n1, n2);
        if let Some(val) = op(n1, n2) {
            self.stack.push(val);
        }
        if n2 == 0 && result.is_none() {
            return Err(Error::DivisionByZero);
        } else {
            Ok(())
        }
    }
}
