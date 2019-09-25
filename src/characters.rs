
pub use crate::enums::{CharKind, CharKind::*};
pub use crate::enums::{Operator, Operator::*};

pub fn categorize_first_of(s: &str) -> Option<CharKind> {
  if let Some(c) = s.chars().next() {
    match c {
      ' ' => Some(Space),
      '(' => Some(LeftParen),
      ')' => Some(RightParen),
      '.' => Some(Dot),
      '-' => Some(Math(Subtract)),
      '+' => Some(Math(Add)),
      '/' => Some(Math(Divide)),
      '*' => Some(Math(Multiply)), // two of these next to each other is actually Math::Exponent
      '0'..='9' => Some(Number),
      
      'a'..='z' | 
      'A'..='Z' | 
      '_' => Some(Alpha),
      
      _ => Some(Other),
    }
  } else {
    None
  }
}