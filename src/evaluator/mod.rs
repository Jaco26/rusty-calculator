#![allow(dead_code, unused_variables)]
// from std
use std::collections::HashMap;
// from crate
use crate::parser::Expression;
// child mods
mod errors;
mod paren_tree;
// from child mods
use errors::EvaluationError;


pub fn evaluate(expr: Expression, prev_evaluated: &HashMap<String, f64>) -> Result<f64, EvaluationError> {
  let paren_tree = paren_tree::ParenTree::from_expression(expr);

  println!("{:#?}", paren_tree);

  Ok(6.4)
}
