use std::{error, fmt};


#[derive(Debug, Clone)]
pub struct EvaluationError;

impl fmt::Display for EvaluationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Evaluation error")
  }
}

impl error::Error for EvaluationError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}