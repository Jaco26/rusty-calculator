use crate::errors::SyntaxError;
use crate::parser::ExpressionNode;
use crate::enums::ExpressionNodeKind;

#[derive(Debug, Clone)]
pub enum ExpressionTreeNodeItem {
  Child(ExpressionNode),
  Parent(ExpressionTreeNode),
}



#[derive(Debug, Clone)]
pub struct ExpressionTreeNode {
  pub items: Vec<ExpressionTreeNodeItem>
}

impl ExpressionTreeNode {
  pub fn new() -> ExpressionTreeNode {
    ExpressionTreeNode {
      items: Vec::new(),
    }
  }

  fn add(&mut self, item: ExpressionTreeNodeItem) {
    self.items.push(item);
  }
}


#[derive(Debug, Clone)]
pub struct ExpressionTree {
  pub root: Option<ExpressionTreeNode>,
}

impl ExpressionTree {
  pub fn new() -> ExpressionTree {
    ExpressionTree {
      root: None,
    }
  }

  pub fn parse(&mut self, exp_nodes: Vec<ExpressionNode>) -> Result<(), SyntaxError> {

    let mut stack = Stack::new();

    stack.push(ExpressionTreeNode::new());

    for node in exp_nodes {
      match node.kind {
        ExpressionNodeKind::Float |
        ExpressionNodeKind::VariableName |
        ExpressionNodeKind::Operator(_) => {
          let item = ExpressionTreeNodeItem::Child(node);
          stack.add_to_last(item);
        },
        ExpressionNodeKind::LeftParen => {
          stack.push(ExpressionTreeNode::new());
        },
        ExpressionNodeKind::RightParen => {
          if let Some(exp_tree_node) = stack.pop() {
            stack.add_to_last(ExpressionTreeNodeItem::Parent(exp_tree_node));
          }
        },
        _ => {},
      }
    }

    if let Some(root) = stack.pop() {
      self.root = Some(root);
    }

    Ok(())
  }
}


#[derive(Debug, Clone)]
struct Stack {
  items: Vec<ExpressionTreeNode>
}

impl Stack {
  fn new() -> Stack {
    Stack {
      items: Vec::new(),
    }
  }

  fn push(&mut self, exp_tree_node: ExpressionTreeNode) {
    self.items.push(exp_tree_node);
  }

  fn pop(&mut self) -> Option<ExpressionTreeNode> {
    self.items.pop()
  }

  fn add_to_last(&mut self, item: ExpressionTreeNodeItem) {
    if let Some(exp_tree_node) = self.items.last_mut() {
      exp_tree_node.add(item);
    }
  }
}