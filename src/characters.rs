
#[derive(Debug, Clone)]
pub enum MathChar {
  Exponent,
  Multiply,
  Divide,
  Add,
  Subtract,
}

#[derive(Debug, Clone)]
pub enum CharKind {
  Space,
  LeftParen,
  RightParen,
  Dot,
  Alpha,
  Number,
  Math(Option<MathChar>)
}

pub use MathChar::*;
pub use CharKind::*;

pub fn categorize_first_of(s: &str) -> Option<CharKind> {
  if let Some(c) = s.chars().next() {
    match c {
      ' ' => Some(Space),
      '(' => Some(LeftParen),
      ')' => Some(RightParen),
      '.' => Some(Dot),
      '-' => Some(Math(Some(Subtract))),
      '+' => Some(Math(Some(Add))),
      '/' => Some(Math(Some(Divide))),
      '*' => Some(Math(None)), // two of these next to each other is actually Math::Exponent
      '0'..='9' => Some(Number),
      
      'a'..='z' | 
      'A'..='Z' | 
      '_' => Some(Alpha),
      
      _ => None,
    }
  } else {
    None
  }
}