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

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNodeKind {
  Float,
  VariableName,
  Operator(MathOperator),
  LeftParen,
  RightParen,
  Space,
  Init,
}