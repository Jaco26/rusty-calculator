use crate::enums::{Operator, Operator::*};

type Action = dyn Fn(f64, f64) -> f64;

pub struct OpsQueueNode {
  action:dyn Fn(f64, f64) -> f64
}

impl OpsQueueNode {
  pub fn new(op_kind: Operator) -> OpsQueueNode {
    let action = match op_kind {
      Exponent => |a: f64, b: f64| a.powf(b),
      Multiply => |a: f64, b: f64| a * b,
      // Add => Box::new(|a, b| a + b),
      // Subtract
    };

  }
}

pub struct OpsQueue {
  exponents: Vec<OpsQueueNode>,
  mult_divide: Vec<OpsQueueNode>,
  add_subtract: Vec<OpsQueueNode>,
}

impl OpsQueue {
  pub fn new() -> OpsQueue {
    OpsQueue {
      exponents: vec![],
      mult_divide: vec![],
      add_subtract: vec![],
    }
  }
}