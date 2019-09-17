#![allow(dead_code, unused_variables)]

use std::collections::HashMap;

pub fn evaluate_expression(expr: Vec<String>, prev_evaluated: &HashMap<String, f64>) -> Result<f64, Box<dyn std::error::Error>> {
  
  println!("In evaluate_expression: {:?}", expr);

  Ok(6.4)
}