
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
  Exponent,
  Multiply,
  Divide,
  Add,
  Subtract,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CharKind {
  Other,
  Space,
  LeftParen,
  RightParen,
  Dot,
  Alpha,
  Number,
  Math(Operator),
}

#[derive(Debug, Clone)]
pub enum EvaluationItem {
  Float(f64),
  StoredVariable(String),
  Math(Operator),
  LeftParen,
  RightParen,
}

