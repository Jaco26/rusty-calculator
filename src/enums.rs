#[derive(Debug, Clone, PartialEq)]
pub enum MathOperator {
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
  Operator(MathOperator)
}

#[derive(Debug, Clone)]
pub enum ExpressionNodeKind {
  Float(f64),
  VariableName(String),
  Operator(MathOperator),
  LeftParen,
  RightParen,
}