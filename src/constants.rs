#![allow(dead_code)]

// Parens
pub const LEFT_PAREN: &str = "(";
pub const RIGHT_PAREN: &str = ")";

// Operators
pub const EXPONENT: &str = "**";
pub const MULTIPLY: &str = "*";
pub const DIVIDE: &str = "/";
pub const ADD: &str = "+";
pub const SUBTRACT: &str = "-";

// Other
pub const SPACE: &str = " ";


// Groups
pub const OPERATORS: [&str; 5] = [
  EXPONENT,
  MULTIPLY,
  DIVIDE,
  ADD,
  SUBTRACT,
];

pub const RESERVED_CHARS: [&str; 7] = [
  LEFT_PAREN,
  RIGHT_PAREN,
  EXPONENT,
  MULTIPLY,
  DIVIDE,
  ADD,
  SUBTRACT,
];