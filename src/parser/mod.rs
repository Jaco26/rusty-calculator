mod structs;

use crate::enums::{CharKind, MathOperator};
use crate::errors::SyntaxError;

pub use structs::UserInputParser;

// fn categorize_char(c: char) -> CharKind {
//   match c {
//     ' ' => CharKind::Space,
//     '(' => CharKind::LeftParen,
//     ')' => CharKind::RightParen,
//     '.' => CharKind::Dot,
//     '-' => CharKind::Operator(MathOperator::Subtract),
//     '+' => CharKind::Operator(MathOperator::Add),
//     '/' => CharKind::Operator(MathOperator::Divide),
//     '*' => CharKind::Operator(MathOperator::Multiply),
//     '0'..='9' => CharKind::Number,

//     'a'..='z' |
//     'A'..='Z' |
//     '_' => CharKind::Alpha,

//     _ => CharKind::Other,
//   }
// }

// pub fn parse_user_input(chars: std::str::Chars) -> Result<Option<ParsedExpression>, SyntaxError> {
//   let categorized_chars = chars.
  
//   let rv = ParsedExpression::new();

//   for c in chars {
//     rv.add(categorize_char(c), String::from(c));
//   }

//   Ok(Some(rv))
// }