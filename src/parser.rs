use crate::enums::{CharKind, MathOperator};

fn categorize_char(c: &char) -> CharKind {
  match c {
    ' ' => CharKind::Space,
    '(' => CharKind::LeftParen,
    ')' => CharKind::RightParen,
    '.' => CharKind::Dot,
    '-' => CharKind::Operator(MathOperator::Subtract),
    '+' => CharKind::Operator(MathOperator::Add),
    '/' => CharKind::Operator(MathOperator::Divide),
    '*' => CharKind::Operator(MathOperator::Multiply),
    '0'..='9' => CharKind::Number,

    'a'..='z' |
    'A'..='Z' |
    '_' => CharKind::Alpha,

    _ => CharKind::Other,
  }
}

