// #[derive(Debug)]
// pub enum CharKind {
//   Math,
//   Dot,
//   LeftParen,
//   RightParen,
//   Number,
//   Alpha,
//   Space,
// }

// use CharKind::*;

// pub fn categorize_first_char_of(s: &str) -> Option<CharKind> {
//   if let Some(c) = s.chars().next() {
//     match c {
//       ' ' => Some(Space),
//       '(' => Some(LeftParen),
//       ')' => Some(RightParen),
//       '.' => Some(Dot),
//       '*' | '/' | '+' | '-' => Some(Math),
//       'a'..='z' | 'A'..='Z' | '_' => Some(Alpha),
//       '0'..='9' => Some(Number),
//       _ => None,
//     }
//   } else {
//     None
//   }
// }