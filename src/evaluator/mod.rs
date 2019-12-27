
mod priority_queue;

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

use priority_queue::{PriorityQueue, PQNode, PQNodeValue };


pub fn evaluate(exp_tree_node: ExpressionTreeNode) -> Result<Option<f64>, RuntimeError> {
  
  let mut queue = PriorityQueue::new();

  let mut pq_node = PQNode::new();

  let mut visited: HashMap<usize, ExpressionNode> = HashMap::new();

  for item in exp_tree_node.items {
    match item {
      ExpressionTreeNodeItem::Parent(node) => {
        // This node has children. PRIORITIZE THIS
        let value = evaluate(node).unwrap();
        if let Some(value) = value {
          pq_node.set_operand(PQNodeValue::PreviouslyEvaluated(value));
        }
      },
      ExpressionTreeNodeItem::Child(node) => {
        visited.insert(node.id, node.clone());
        // This node is either a MathOperator, Float
        match node.kind {
          ExpressionNodeKind::Operator(math_op)=> {
            pq_node.set_operator(math_op);
          },
          ExpressionNodeKind::Float => {
            pq_node.set_operand(PQNodeValue::Leaf(node));
          },
          _ => {},
        }
      }
    }

      // if the priority queue node is full, enqueue it
    if pq_node.is_full() == true {
      pq_node.set_priority();
      queue.enqueue(pq_node);
      pq_node = PQNode::new();
    }
  }

  // println!("Hello {:#?}", queue);

  Ok(Some(0.0))
}
