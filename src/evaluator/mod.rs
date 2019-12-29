
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

  // store the indices of visited expression_tree_node_items
  // let mut visited: HashMap<usize, &bool> = HashMap::new();
  let mut visited = HashSet::new();

  let mut queue_stack = PriorityQueue::new();

  // For the given ExpressionTreeNode, get a list of all indices of 
  // ExpressionNode::Operator nodes
  let operator_indices = OperatorIndices::new(&exp_tree_node.items);

  let mut eval_node = EvalNode::new();

  for i in operator_indices.add_subtract {
    let operator = exp_tree_node.items[i].clone();
    let left_of_operator = exp_tree_node.items[i - 1].clone();
    let right_of_operator = exp_tree_node.items[i + 1].clone();

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
        let evaluated = evaluate(exp_tree_node).unwrap().unwrap();
        if eval_node.left == EvalNodeOperand::Init {
          eval_node.left = EvalNodeOperand::Float(evaluated);
        } else if eval_node.right == EvalNodeOperand::Init {
          eval_node.right = EvalNodeOperand::Float(evaluated);
        }
      },
      ExpressionTreeNodeItem::Child(exp_node) => {
        if visited.contains(&exp_node.id) {
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
          visited.insert(exp_node.id);
        }
      }
    }

    
    match right_of_operator {
      ExpressionTreeNodeItem::Parent(exp_tree_node) => {
        let evaluated = evaluate(exp_tree_node).unwrap().unwrap();
        if eval_node.left == EvalNodeOperand::Init {
          eval_node.left = EvalNodeOperand::Float(evaluated);
        } else if eval_node.right == EvalNodeOperand::Init {
          eval_node.right = EvalNodeOperand::Float(evaluated);
        }
      },
      ExpressionTreeNodeItem::Child(exp_node) => {
        if visited.contains(&exp_node.id) {
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
          visited.insert(exp_node.id);
        }
      }
    }

    queue_stack.enqueue(eval_node);
    eval_node = EvalNode::new();
  }

  println!("{:#?}", queue_stack);

  let mut accum = 0.0;
  while queue_stack.items.len() > 0 {
    let mut dequeued = queue_stack.dequeue();
    accum += dequeued.evaluate();
  }

  Ok(Some(accum))
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
    // self.items.sort_by(|a, b| a.priority.cmp(&b.priority));
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