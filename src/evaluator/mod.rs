#![allow(dead_code, unused_variables)]

mod errors;

use std::collections::HashMap;
use errors::EvaluationError;
use crate::parser::Expression;

pub fn evaluate(exp: Expression, prev_evaluated: &HashMap<String, f64>) -> Result<f64, EvaluationError> {
  
  Ok(6.4)
}
