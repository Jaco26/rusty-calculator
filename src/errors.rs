use std::{error, fmt};

#[derive(Debug, Clone)]
pub enum SyntaxError {
  EmptyParens,
  UnbalancedParens,
  AdjacentMathOperators,
  OperatorBeginsScope,
  OperatorEndsScope,
}

#[derive(Debug, Clone)]
pub enum RuntimeError {
  VariableIsUndfined,
}

impl fmt::Display for SyntaxError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}

impl error::Error for SyntaxError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

impl fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}

impl error::Error for RuntimeError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}