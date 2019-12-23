// from std
use std::collections::HashMap;
// from crate
use crate::parser::Expression;
use crate::enums::{
  EvaluationItem,
  EvaluationItem::*
};
// from parent mod
use super::errors::EvaluationError;

#[derive(Debug)]
pub enum ParenNodeItem {
  EvalItem(EvaluationItem),
  Paren(ParenNode),
}


#[derive(Debug)]
pub struct ParenNode {
  items: Vec<ParenNodeItem>,
}

impl ParenNode {
  pub fn new() -> ParenNode {
    ParenNode {
      items: Vec::new(),
    }
  }
  pub fn add(&mut self, item: ParenNodeItem) {
    self.items.push(item);

  }
}


struct ParenChildrenStack {
  _items: Vec<ParenNode>,
}

impl ParenChildrenStack {
  fn new() -> ParenChildrenStack {
    ParenChildrenStack {
      _items: vec![],
    }
  }
  fn push(&mut self, value: ParenNode) {
    self._items.push(value);
  }
  fn pop(&mut self) -> Option<ParenNode> {
    self._items.pop()
  }
  fn add_to_last(&mut self, item: ParenNodeItem) {
    if let Some(node) = self._items.last_mut() {
      node.add(item);
    }
  }
}

#[derive(Debug)]
pub struct ParenTree {
  pub root: ParenNode
}

impl ParenTree {
  pub fn from_expression(expression: Expression) -> Option<ParenTree> {

    let mut stack = ParenChildrenStack::new();

    stack.push(ParenNode::new());
    
    for x in expression {
      if let Some(eval_item) = x.to_evaluation_item() {
         match eval_item {
          Float(_) |
          StoredVariable(_) |
          Math(_) => {
            let item = ParenNodeItem::EvalItem(eval_item);
            stack.add_to_last(item);
          },
          LeftParen => {
            stack.push(ParenNode::new());
          },
          RightParen => {
            if let Some(paren_node) = stack.pop() {
              stack.add_to_last(ParenNodeItem::Paren(paren_node));
            }
          }
        }
      }
    }

    if let Some(root) = stack.pop() {
      return Some(ParenTree { root });
    };
    None
  }
}
