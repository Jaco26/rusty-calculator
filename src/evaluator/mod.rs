
mod eval_node;

use std::collections::HashMap;

use crate::errors::RuntimeError;
use crate::enums::{
  MathOperator,
  ExpressionNodeKind
};
use crate::parser::{
  ExpressionNode,
  expression_tree::{
    ExpressionTree,
    ExpressionTreeNode,
    ExpressionTreeNodeItem
  }
};

use eval_node::{EvalNode, EvalNodeOperand};


pub fn evaluate(exp_tree_node: ExpressionTreeNode) -> Result<Option<f64>, RuntimeError> {

  let mut visited: HashMap<usize, ExpressionNode> = HashMap::new();

  let mut queue = PriorityQueue::new();

  let mut eval_node = EvalNode::new();

  for node_item in exp_tree_node.items {
    match node_item {
      ExpressionTreeNodeItem::Parent(exp_tree_node) => {
        let evaluated_paren = evaluate(exp_tree_node).unwrap();
        eval_node.set_operand(EvalNodeOperand::Float(evaluated_paren.unwrap()));
      },
      ExpressionTreeNodeItem::Child(exp_node) => {
        match exp_node.kind {
          ExpressionNodeKind::Operator(math_op) => {
            eval_node.set_operator(math_op);
          },
          ExpressionNodeKind::Float => {
            eval_node.set_operand(EvalNodeOperand::Float(exp_node.value.parse().unwrap()))
          },
          _ => {},
        }
      },
    }

    if eval_node.is_full() {
      queue.enqueue(eval_node);
      eval_node = EvalNode::new();
    }
  }

  let mut accum = 0.0;
  while queue.items.len() > 0 {
    let mut dequeued = queue.dequeue();
    accum += dequeued.evaluate();
  }

  Ok(Some(accum))
}



struct PriorityQueue { 
  items: Vec<EvalNode>
}

impl PriorityQueue {
  fn new() -> PriorityQueue {
    PriorityQueue { items: Vec::new() }
  }

  fn enqueue(&mut self, eval_node: EvalNode) {
    self.items.push(eval_node);
    self.items.sort_by(|a, b| a.priority.cmp(&b.priority));
  }

  fn dequeue(&mut self) -> EvalNode {
    self.items.remove(0)
  }
}
