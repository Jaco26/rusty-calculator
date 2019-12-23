use crate::enums::{ExpressionNodeKind, CharKind};

#[derive(Debug, Clone)]
pub struct ExpressionChar {
  kind: CharKind,
  value: String,
}

impl ExpressionChar {
  pub fn new(k: CharKind, v: String) -> ExpressionChar {
    ExpressionChar {
      kind: k,
      value: v,
    }
  }
}


#[derive(Debug, Clone)]
pub struct ExpressionNode {
  kind: ExpressionNodeKind,
  value: String,
}

impl ExpressionNode {
  pub fn new(k: ExpressionNodeKind, v: String) -> ExpressionNode {
    ExpressionNode {
      kind: k,
      value: v,
    }
  }
}


#[derive(Debug, Clone)]
pub struct ExpressionTree;