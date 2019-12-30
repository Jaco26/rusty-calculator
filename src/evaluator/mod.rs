
mod eval_node;

use std::collections::HashSet;

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

  let mut visited: HashSet<usize> = HashSet::new();

  let mut queue_stack = PriorityQueue::new();

  // For the given ExpressionTreeNode, get a list of all indices of ExpressionNode::Operator nodes
  let operator_indices = OperatorIndices::new(&exp_tree_node.items);

  process_operator_indices(&exp_tree_node, operator_indices.exponents, &mut visited, &mut queue_stack)?;

  process_operator_indices(&exp_tree_node, operator_indices.mult_divide, &mut visited, &mut queue_stack)?;

  process_operator_indices(&exp_tree_node, operator_indices.add_subtract, &mut visited, &mut queue_stack)?;
  
  // println!("{:#?}", queue_stack);

  let mut accum = 0.0;
  while queue_stack.items.len() > 0 {
    let mut dequeued = queue_stack.dequeue();
    accum += dequeued.evaluate();
  }

  Ok(Some(accum))
}



fn process_operator_indices(
  exp_tree_node: &ExpressionTreeNode,
  indices: Vec<usize>,
  visited: &mut HashSet<usize>,
  queue_stack: &mut PriorityQueue,
) -> Result<(), RuntimeError> {

  let mut eval_node = EvalNode::new();

  for i in indices {
    let left_i = i - 1;
    let right_i = i + 1;
    let operator = exp_tree_node.items[i].clone();
    let left_of_operator = exp_tree_node.items[left_i].clone();
    let right_of_operator = exp_tree_node.items[right_i].clone();

    if let ExpressionTreeNodeItem::Child(exp_node) = operator {
      match exp_node.kind {
        ExpressionNodeKind::Operator(math_op) => {
          eval_node.set_operator(math_op);
        },
        _ => {},
      }
    };

    match left_of_operator {
      ExpressionTreeNodeItem::Parent(exp_tree_node) => {
        if visited.contains(&left_i) {
          if eval_node.left == EvalNodeOperand::Init {
            eval_node.left = EvalNodeOperand::Float(0.0);
          } else if eval_node.right == EvalNodeOperand::Init {
            eval_node.right = EvalNodeOperand::Float(0.0);
          }
        } else {
          let evaluated = evaluate(exp_tree_node).unwrap().unwrap();
          if eval_node.left == EvalNodeOperand::Init {
            eval_node.left = EvalNodeOperand::Float(evaluated);
          } else if eval_node.right == EvalNodeOperand::Init {
            eval_node.right = EvalNodeOperand::Float(evaluated);
          }
        }
        visited.insert(left_i);
      },
      ExpressionTreeNodeItem::Child(exp_node) => {
        if visited.contains(&left_i) {
          let last_added = queue_stack.pop().unwrap(); // NEED WAY OF TARGETING CORRECT QUEUED EVAL ITEM
          if eval_node.left == EvalNodeOperand::Init {
            eval_node.left = EvalNodeOperand::EvalNode(Box::new(last_added));
          } else if eval_node.right == EvalNodeOperand::Init {
            eval_node.right = EvalNodeOperand::EvalNode(Box::new(last_added));
          }
        } else {
          match exp_node.kind {
            ExpressionNodeKind::Float => {
              let parsed = exp_node.value.parse().unwrap();
              if eval_node.left == EvalNodeOperand::Init {
                eval_node.left = EvalNodeOperand::Float(parsed);
              } else if eval_node.right == EvalNodeOperand::Init {
                eval_node.right = EvalNodeOperand::Float(parsed);
              }
            },
            _ => {}
          }
          visited.insert(left_i);
        }
      }
    }

    
    match right_of_operator {
      ExpressionTreeNodeItem::Parent(exp_tree_node) => {
        if visited.contains(&right_i) {
          if eval_node.left == EvalNodeOperand::Init {
            eval_node.left = EvalNodeOperand::Float(0.0);
          } else if eval_node.right == EvalNodeOperand::Init {
            eval_node.right = EvalNodeOperand::Float(0.0);
          }
        } else {
          let evaluated = evaluate(exp_tree_node).unwrap().unwrap();
          if eval_node.left == EvalNodeOperand::Init {
            eval_node.left = EvalNodeOperand::Float(evaluated);
          } else if eval_node.right == EvalNodeOperand::Init {
            eval_node.right = EvalNodeOperand::Float(evaluated);
          }
          visited.insert(right_i);
        }
      },
      ExpressionTreeNodeItem::Child(exp_node) => {
        if visited.contains(&right_i) {
          let last_added = queue_stack.pop().unwrap(); // NEED WAY OF TARGETING CORRECT QUEUED EVAL ITEM
          if eval_node.left == EvalNodeOperand::Init {
            eval_node.left = EvalNodeOperand::EvalNode(Box::new(last_added));
          } else if eval_node.right == EvalNodeOperand::Init {
            eval_node.right = EvalNodeOperand::EvalNode(Box::new(last_added));
          }
        } else {
          match exp_node.kind {
            ExpressionNodeKind::Float => {
              let parsed = exp_node.value.parse().unwrap();
              if eval_node.left == EvalNodeOperand::Init {
                eval_node.left = EvalNodeOperand::Float(parsed);
              } else if eval_node.right == EvalNodeOperand::Init {
                eval_node.right = EvalNodeOperand::Float(parsed);
              }
            },
            _ => {}
          }
          visited.insert(right_i);
        }
      }
    }

    queue_stack.enqueue(eval_node);
    eval_node = EvalNode::new();
  }

  Ok(())
}







#[derive(Debug)]
struct PriorityQueue { 
  items: Vec<EvalNode>
}

impl PriorityQueue {
  fn new() -> PriorityQueue {
    PriorityQueue { items: Vec::new() }
  }

  fn enqueue(&mut self, eval_node: EvalNode) {
    self.items.push(eval_node);
  }

  fn dequeue(&mut self) -> EvalNode {
    self.items.remove(0)
  }

  fn pop(&mut self) -> Option<EvalNode> {
    self.items.pop()
  }
}


struct OperatorIndices {
  exponents: Vec<usize>,
  mult_divide: Vec<usize>,
  add_subtract: Vec<usize>,
}

impl OperatorIndices {
  fn new(items: &Vec<ExpressionTreeNodeItem>) -> OperatorIndices {
    let mut exponents = Vec::new();
    let mut mult_divide = Vec::new();
    let mut add_subtract = Vec::new();

    for (i, exp_tree_node_item) in items.iter().enumerate() {
      match exp_tree_node_item {
        ExpressionTreeNodeItem::Child(exp_node) => {
          if let ExpressionNodeKind::Operator(math_op) = &exp_node.kind {
            match math_op {
              MathOperator::Exponent => exponents.push(i),

              MathOperator::Multiply |
              MathOperator::Divide => mult_divide.push(i),

              MathOperator::Add |
              MathOperator::Subtract => add_subtract.push(i),
            }
          }
        },
        _ => {},
      }
    }

    OperatorIndices { exponents, mult_divide, add_subtract }
  }
}