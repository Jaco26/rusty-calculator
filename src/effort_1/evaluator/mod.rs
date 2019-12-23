#![allow(dead_code, unused_variables)]
// from std
use std::collections::HashMap;
// from crate
use crate::parser::Expression;
// child mods
mod errors;
mod paren_tree;
mod evaluation_node;
mod order_of_ops;
// from child mods
use errors::EvaluationError;
use paren_tree::{ParenTree, ParenNode};
use evaluation_node::EvaluationNode;
use order_of_ops::OperationsQueue;



pub fn evaluate(expr: Expression, prev_evaluated: &HashMap<String, f64>) -> Result<f64, EvaluationError> {
  let paren_tree = ParenTree::from_expression(expr);

  

  match paren_tree {
    Some(paren_tree) => {

      // let queue = vec![]; // push 
      
      // Ok(result)
    },
    None => Err(EvaluationError),
  }
}
