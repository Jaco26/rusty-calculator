#[derive(Debug)]
pub enum CharKind {
  Math,
  Dot,
  LeftParen,
  RightParen,
  Number,
  Alpha,
  Space,
}

use CharKind::*;

pub fn categorize_first_char_of(s: &str) -> Option<CharKind> {
  if let Some(c) = s.chars().next() {
    match c {
      '*' | '/' | '+' | '-' => Some(Math),
      '(' => Some(LeftParen),
      ')' => Some(RightParen),
      '.' => Some(Dot),
      ' ' => Some(Space),
      'a'..='z' | 'A'..='Z' => Some(Alpha),
      _ => {
        if c.is_numeric() {
          Some(Number)
        } else {
          None
        }
      }
    }
  } else {
    None
  }
}