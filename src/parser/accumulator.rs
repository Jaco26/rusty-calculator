use std::collections::HashMap;
use crate::enums::{CharKind, CharKind::*};


#[derive(Debug, Clone)]
pub struct ExpressionNodeItem {
  _value: String,
  _kind: CharKind,
}

impl ExpressionNodeItem {
  pub fn new(val: &str, kind: CharKind) -> ExpressionNodeItem {
    ExpressionNodeItem {
      _value: String::from(val),
      _kind: kind,
    }
  }
  pub fn value(&self) -> String {
    String::from(&self._value)
  }
  pub fn kind(&self) -> CharKind {
    self._kind.clone()
  }
}


#[derive(Debug, Clone)]
pub struct ExpressionNode {
  _contents: Vec<ExpressionNodeItem>,
}

impl ExpressionNode {
  pub fn new() -> ExpressionNode {
    ExpressionNode {
      _contents: Vec::new(),
    }
  }
  pub fn add(&mut self, item: ExpressionNodeItem) {
    self._contents.push(item);
  }
  pub fn contents(&self) -> Vec<ExpressionNodeItem> {
    self._contents.clone()
  }
}

pub struct Accumulator {
  _buffer: ExpressionNode,
  _values: Vec<ExpressionNode>
}

impl Accumulator {
  pub fn new() -> Accumulator {
    Accumulator {
      _buffer: ExpressionNode::new(),
      _values: Vec::new(),
    }
  }

  pub fn parens_are_balanced(&self) -> bool {
    let mut accum: HashMap<&str, usize> = HashMap::new();
    accum.insert("n_left", 0);
    accum.insert("n_right", 0);

    let accum = self._values.iter().fold(accum, |mut acc, x| {
      if let Some(accum_node_item) = x._contents.iter().next() {
        match accum_node_item.kind() {
          LeftParen => {
            acc.insert("n_left", acc.get("n_left").unwrap() + 1);
          },
          RightParen => {
            acc.insert("n_right", acc.get("n_right").unwrap() + 1);
          },
          _ => {},
        };
      }
      acc
    });

    accum.get("n_left").unwrap() == accum.get("n_right").unwrap()
  }

  pub fn add_item(&mut self, item: ExpressionNodeItem) {
    self.flush_buffer();
    self.add_to_buffer(item);
    self.flush_buffer();
  }

  pub fn add_to_buffer(&mut self, item: ExpressionNodeItem) {
    self._buffer.add(item);
  }

  pub fn flush_buffer(&mut self) {
    if self._buffer.contents().len() > 0 {
      self._values.push(self._buffer.clone());
      self._buffer = ExpressionNode::new();
    }
  }

  pub fn lookback_char_kind(&self) -> Option<CharKind> {
    if let Some(item) = self._buffer.contents().last() {
      return Some(item.kind());
    }

    if let Some(accum_node) = self._values.last() {
      if let Some(item) = accum_node.contents().last() {
        return Some(item.kind());
      }
    }
    None
  }

  pub fn values(&self) -> Option<Vec<ExpressionNode>> {
    if self._values.len() > 0 {
      let collected: Vec<ExpressionNode> = self._values.iter()
        .map(|x| x.clone())
        .collect();
      Some(collected)
    } else {
      None
    }
  }

  pub fn pop_values(&mut self) -> Option<ExpressionNode> {
    self._values.pop()
  }
}