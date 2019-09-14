use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParserError {
  BadVarName(&'static str),
  EmptyExpression,
}


impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "")
  }
}


impl error::Error for ParserError {
  // necessary for other errors to wrap this one
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    // this is just a generic error
    None
  }
}