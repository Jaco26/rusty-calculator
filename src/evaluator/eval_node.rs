
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
  pub left: EvalNodeOperand,
  pub right: EvalNodeOperand,
}

impl EvalNode {
  pub fn new() -> EvalNode {
    EvalNode {
      operator: None,
      left: EvalNodeOperand::Init,
      right: EvalNodeOperand::Init,
    }
  }

  pub fn set_operator(&mut self, op: MathOperator) {
    self.operator = Some(op);
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