
use crate::enums::MathOperator;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalNodeOperand {
  Float(f64),
  EvalNode(Box<EvalNode>),
  Init,
}


#[derive(Debug, Clone, PartialEq)]
pub struct EvalNode {
  operator: Option<MathOperator>,
  pub priority: usize,
  pub left: EvalNodeOperand,
  pub right: EvalNodeOperand,
}

impl EvalNode {
  pub fn new() -> EvalNode {
    EvalNode {
      priority: 0,
      operator: None,
      left: EvalNodeOperand::Init,
      right: EvalNodeOperand::Init,
    }
  }

  pub fn set_operator(&mut self, op: MathOperator) {
    let priority = match op {
      MathOperator::Exponent => 1,

      MathOperator::Multiply |
      MathOperator::Divide => 2,

      MathOperator::Add |
      MathOperator::Subtract => 3,
    };
    self.priority = priority;
    self.operator = Some(op);
  }

  pub fn set_operand(&mut self, operand: EvalNodeOperand) {
    if EvalNodeOperand::Init == self.left {
      self.left = operand;
    } else if EvalNodeOperand::Init == self.right {
      self.right = operand;
    }
  }

  pub fn is_full(&self) -> bool {
    if self.left != EvalNodeOperand::Init && self.right != EvalNodeOperand::Init && self.operator != None {
      true
    } else {
      false
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
      Some(MathOperator::Exponent) => left.powf(right),
      Some(MathOperator::Multiply) => left * right,
      Some(MathOperator::Divide) => left / right,
      Some(MathOperator::Add) => left + right,
      Some(MathOperator::Subtract) => left - right,
      None => {
        eprintln!("cannot evalute because self.operator is None");
        0.0
      }
    }
  }
}