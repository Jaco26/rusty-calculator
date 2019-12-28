
use crate::enums::MathOperator;

#[derive(Debug, Clone)]
pub enum EvalNodeOperand {
  Float(f64),
  EvalNode(Box<EvalNode>),
  Init,
}


#[derive(Debug, Clone)]
pub struct EvalNode {
  operator: MathOperator,
  pub priority: usize,
  pub left: EvalNodeOperand,
  pub right: EvalNodeOperand,
}

impl EvalNode {
  pub fn new(op: MathOperator) -> EvalNode {
    let priority = match op {
      MathOperator::Exponent => 1,

      MathOperator::Multiply |
      MathOperator::Divide => 2,

      MathOperator::Add |
      MathOperator::Subtract => 3,
    };

    EvalNode {
      priority,
      operator: op,
      left: EvalNodeOperand::Init,
      right: EvalNodeOperand::Init,
    }
  }

  pub fn evaluate(&mut self) -> f64 {

    if let EvalNodeOperand::EvalNode(mut boxed_eval_node) = self.left.clone() {
      self.left = EvalNodeOperand::Float(boxed_eval_node.evaluate());
    }

    if let EvalNodeOperand::EvalNode(mut boxed_eval_node) = self.right.clone() {
      self.right = EvalNodeOperand::Float(boxed_eval_node.evaluate());
    }

    let left = match self.left {
      EvalNodeOperand::Float(f) => f,
      _ => {
        eprintln!("Couldn't get a f64 value for EvalNode.left");
        0.0
      }
    };

    let right = match self.right {
      EvalNodeOperand::Float(f) => f,
      _ => {
        eprintln!("Couldn't get a f64 value for EvalNode.right");
        0.0
      }
    };

    match self.operator {
      MathOperator::Exponent => left.powf(right),
      MathOperator::Multiply => left * right,
      MathOperator::Divide => left / right,
      MathOperator::Add => left + right,
      MathOperator::Subtract => left - right,
    }
  }
}