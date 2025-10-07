use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum OperationType {
    NONE,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

#[derive(PartialEq, Clone, Copy)]
enum Token {
    Operation(OperationType),
    Operand(i32),
    ReservedWord,
    Unknown,
}

static KNOWN_WORD_LIST: [&'static str; 4] = ["what", "is", "the", "by"];

fn perform_operation(operation: OperationType, a: i32, b: i32) -> Option<i32>{
    match operation {
        OperationType::NONE => None,
        OperationType::PLUS => Some(a + b),
        OperationType::MINUS => Some(a - b),
        OperationType::MULTIPLY => Some(a * b),
        OperationType::DIVIDE => Some(a / b),
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let operation_mapping = HashMap::from([
        ("plus", OperationType::PLUS),
        ("minus", OperationType::MINUS),
        ("multiplied", OperationType::MULTIPLY),
        ("divided", OperationType::DIVIDE),
    ]);

    let lowercased = command.to_lowercase();
    let replaced = lowercased.replace("?", "");

    let mut tokens = replaced
        .split(' ')
        .map(|token| {
            if let Some(operation) = operation_mapping.get(token) {
               Token::Operation(*operation)
            } else if let Ok(digits) = token.parse::<i32>() {
                Token::Operand(digits)
            } else if KNOWN_WORD_LIST.contains(&token) {
                Token::ReservedWord
            } else {
                Token::Unknown
            }
        })
        .filter(|token| *token != Token::ReservedWord);
    
    let mut sum = if let Some(Token::Operand(val)) = tokens.next() { Some(val) } else { None };
    let mut current_operation = OperationType::NONE;
    for token in tokens {
        if let Token::Unknown = token {
            sum = None;
        }
        if let (Token::Operand(digits), Some(acc)) = (token, sum) {
            sum = perform_operation(current_operation, acc, digits);
            current_operation = OperationType::NONE;
        } 
        if let Token::Operation(op) = token {
            if current_operation == OperationType::NONE {
                current_operation = op;
            } else {
                sum = None
            }
        }
    }
    if current_operation != OperationType::NONE { None } else { sum }
}
