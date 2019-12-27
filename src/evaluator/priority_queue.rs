use crate::parser::ExpressionNode;
use crate::enums::MathOperator;

#[derive(Debug)]
pub struct PriorityQueue {
  items: Vec<PQNode>,
}

impl PriorityQueue {
  pub fn new() -> PriorityQueue {
    PriorityQueue { items: Vec::new() }
  }

  pub fn enqueue(&mut self, node: PQNode) {
    self.items.push(node);
    self.items.sort_by(|a, b| a.priority.cmp(&b.priority));
  }

  pub fn dequeue(&mut self) -> PQNode {
    self.items.remove(0)
  }
}

#[derive(Debug, PartialEq)]
pub enum PQNodeValue {
  Leaf(ExpressionNode),
  PreviouslyEvaluated(f64),
  Recursive(Box<PQNodeValue>),
}

#[derive(Debug)]
pub struct PQNode {
  left: Option<PQNodeValue>,
  right: Option<PQNodeValue>,
  operator: Option<MathOperator>,
  evaluated_value: Option<f64>,
  priority: Option<i32>,
}

impl PQNode {
  pub fn new() -> PQNode {
    PQNode { 
      left: None,
      right: None,
      operator: None,
      priority: None,
      evaluated_value: None,
    }
  }

  pub fn is_full(&self) -> bool {
    if self.left != None && self.right != None && self.operator != None {
      true
    } else {
      false
    }
  }

  pub fn set_operand(&mut self, v: PQNodeValue) {
    if let None = self.left {
      self.left = Some(v);
    } else if let None = self.right {
      self.right = Some(v);
    }
  }

  pub fn set_operator(&mut self, o: MathOperator) {
    self.operator = Some(o);
  }

  pub fn set_priority(&mut self) {
    match self.operator {
      Some(MathOperator::Add) |
      Some(MathOperator::Subtract) => self.priority = Some(3),

      Some(MathOperator::Multiply) |
      Some(MathOperator::Divide) => self.priority = Some(2),

      Some(MathOperator::Exponent) => self.priority = Some(1),

      None => {
        eprintln!("Cannot set PQNode priority because PQNode 'operator' is None");
      }
    };
  }
}