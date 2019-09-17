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

  pub fn add_to_buffer(&mut self, item: AccumNodeItem) {
    self._buffer.add(item);
  }

  pub fn flush_buffer(&mut self) {
    if let Some(contents) = self._buffer.contents() {
      self._values.push(self._buffer.clone());
      self._buffer = AccumNode::new();
    }
  }

  pub fn get_last_added(&self) -> Option<AccumNodeItem> {
    if let Some(buffer_items) = self._buffer.contents() {

      return Some(buffer_items[buffer_items.len() - 1].clone());

    } else if let Some(last) = self._values.last() {
      
      if let Some(contents) = last.contents() {
        return Some(contents[contents.len() - 1].clone());
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
}