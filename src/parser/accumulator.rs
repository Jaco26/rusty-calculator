use std::collections::HashMap;
use crate::characters::*;

#[derive(Debug, Clone)]
pub struct AccumNodeItem {
  _value: String,
  _kind: CharKind,
}

impl AccumNodeItem {
  pub fn new(val: &str, kind: CharKind) -> AccumNodeItem {
    AccumNodeItem {
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
pub struct AccumNode {
  _contents: Vec<AccumNodeItem>,
}

impl AccumNode {
  pub fn new() -> AccumNode {
    AccumNode {
      _contents: Vec::new(),
    }
  }
  pub fn add(&mut self, item: AccumNodeItem) {
    self._contents.push(item);
  }
  pub fn contents(&self) -> Option<Vec<AccumNodeItem>> {
    if self._contents.len() > 0 {
      let contents: Vec<AccumNodeItem> = self._contents.iter()
        .map(|item| item.clone())
        .collect();

      return Some(contents);
    }
    None
  }
}

pub struct Accumulator {
  _buffer: AccumNode,
  _values: Vec<AccumNode>
}

impl Accumulator {
  pub fn new() -> Accumulator {
    Accumulator {
      _buffer: AccumNode::new(),
      _values: Vec::new(),
    }
  }

  pub fn parens_are_balanced(&self) -> bool {
    let mut accum: HashMap<&str, usize> = HashMap::new();
    accum.insert("n_left", 0);
    accum.insert("n_right", 0);

    let accum = self._values.iter().fold(accum, |mut acc, x| {
      if let Some(contents) = x.contents() {
        if let Some(accum_node_item) = contents.iter().next() { // get the first item of x.contents()
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
      }
      acc
    });

    accum.get("n_left").unwrap() == accum.get("n_right").unwrap()
  }

  pub fn add_item(&mut self, item: AccumNodeItem) {
    self.flush_buffer();
    self.add_to_buffer(item);
    self.flush_buffer();
  }

  pub fn add_to_buffer(&mut self, item: AccumNodeItem) {
    self._buffer.add(item);
  }

  pub fn flush_buffer(&mut self) {
    if let Some(_) = self._buffer.contents() {
      self._values.push(self._buffer.clone());
      self._buffer = AccumNode::new();
    }
  }

  pub fn lookback_char_kind(&self) -> Option<CharKind> {
    if let Some(buffer_items) = self._buffer.contents() {
      if let Some(item) = buffer_items.last() {
        return Some(item.kind());
      }
    } else if let Some(accum_node) = self._values.last() {
      if let Some(items) = accum_node.contents() {
        if let Some(item) = items.last() {
          return Some(item.kind());
        }
      }
    }
    None
  }


  pub fn values(&self) -> Option<Vec<AccumNode>> {
    if self._values.len() > 0 {
      let collected: Vec<AccumNode> = self._values.iter()
        .map(|x| x.clone())
        .collect();
      Some(collected)
    } else {
      None
    }
  }

  pub fn pop_values(&mut self) -> Option<AccumNode> {
    self._values.pop()
  }
}