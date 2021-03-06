
mod eval_node;

use std::collections::{HashSet, HashMap};

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


pub fn evaluate(evaluated: &HashMap<String, f64>, exp_tree_node: ExpressionTreeNode) -> Result<Option<f64>, RuntimeError> {

  if exp_tree_node.items.len() == 1 {
    match exp_tree_node.items[0].clone() {
      ExpressionTreeNodeItem::Child(exp_node) => {
        match exp_node.kind {
          ExpressionNodeKind::VariableName => {
            let stored_value = evaluated.get(&exp_node.value);
            if let Some(value) = stored_value {
              return Ok(Some(*value));
            } else {
              return Err(RuntimeError::VariableIsUndfined);
            }
          },
          ExpressionNodeKind::Float => {
            return Ok(Some(exp_node.value.parse().unwrap()));
          },
          _ => {},
        }
      },
      _ => {},
    }
  }

  let mut visited: HashSet<usize> = HashSet::new();

  let mut queue_stack = PriorityQueue::new();

  // For the given ExpressionTreeNode, get a list of all indices of ExpressionNode::Operator nodes
  let operator_indices = OperatorIndices::new(&exp_tree_node.items);

  process_operator_indices(
    evaluated,
    &exp_tree_node,
    operator_indices.exponents,
    &mut visited,
    &mut queue_stack
  )?;

  process_operator_indices(
    evaluated,
    &exp_tree_node,
    operator_indices.mult_divide,
    &mut visited,
    &mut queue_stack
  )?;

  process_operator_indices(
    evaluated,
    &exp_tree_node,
    operator_indices.add_subtract,
    &mut visited,
    &mut queue_stack
  )?;

  let mut accum = 0.0;
  while queue_stack.items.len() > 0 {
    let mut dequeued = queue_stack.dequeue();
    accum += dequeued.evaluate();
  }

  Ok(Some(accum))
}



fn process_operator_indices(
  evaluated: &HashMap<String, f64>,
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

    if let ExpressionTreeNodeItem::Child(exp_node) = operator {
      match exp_node.kind {
        ExpressionNodeKind::Operator(math_op) => {
          eval_node.set_operator(math_op);
        },
        _ => {},
      }
    };

    handle_operand(left_i, &exp_tree_node, &mut eval_node, visited, queue_stack, evaluated)?;
    handle_operand(right_i, &exp_tree_node, &mut eval_node, visited, queue_stack, evaluated)?;

    queue_stack.enqueue(eval_node);
    eval_node = EvalNode::new();
  }

  Ok(())
}


fn handle_operand(
  operand_index: usize,
  exp_tree_node: &ExpressionTreeNode,
  eval_node: &mut EvalNode,
  visited: &mut HashSet<usize>,
  queue_stack: &mut PriorityQueue,
  evaluated: &HashMap<String, f64>,
) -> Result<(), RuntimeError> {

  let mut operand = exp_tree_node.items[operand_index].clone();

  match operand {
    ExpressionTreeNodeItem::Parent(exp_tree_node) => {
      if visited.contains(&operand_index) {
        if eval_node.left == EvalNodeOperand::Init {
          eval_node.left = EvalNodeOperand::Float(0.0);
        } else if eval_node.right == EvalNodeOperand::Init {
          eval_node.right = EvalNodeOperand::Float(0.0);
        }
      } else {
        let evaluated = evaluate(evaluated, exp_tree_node).unwrap().unwrap();
        if eval_node.left == EvalNodeOperand::Init {
          eval_node.left = EvalNodeOperand::Float(evaluated);
        } else if eval_node.right == EvalNodeOperand::Init {
          eval_node.right = EvalNodeOperand::Float(evaluated);
        }
      }
      visited.insert(operand_index);
    },
    ExpressionTreeNodeItem::Child(exp_node) => {
      if visited.contains(&operand_index) {
        let last_added = &queue_stack.pop().unwrap(); // NEED WAY OF TARGETING CORRECT QUEUED EVAL ITEM
        if eval_node.left == EvalNodeOperand::Init {
          eval_node.left = EvalNodeOperand::EvalNode(Box::new(last_added.clone()));
        } else if eval_node.right == EvalNodeOperand::Init {
          eval_node.right = EvalNodeOperand::EvalNode(Box::new(last_added.clone()));
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
          ExpressionNodeKind::VariableName => {
            let stored_value = evaluated.get(&exp_node.value);
            if let Some(value) = stored_value {
              if eval_node.left == EvalNodeOperand::Init {
                eval_node.left = EvalNodeOperand::Float(*value);
              } else if eval_node.right == EvalNodeOperand::Init {
                eval_node.right = EvalNodeOperand::Float(*value);
              }
            } else {
              return Err(RuntimeError::VariableIsUndfined);
            }
          },
          _ => {}
        }
        visited.insert(operand_index);
      }
    }
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