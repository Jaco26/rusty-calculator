use crate::enums::ExpressionNodeKind;

#[derive(Debug)]
pub enum ParenNodeItem {
  Child(ExpressionNodeKind),
  Parent(ParenNode),
}



#[derive(Debug)]
pub struct ParenNode {
  items: Vec<ParenNodeItem>,
}

impl ParenNode {
  pub fn new() -> ParenNode {
    ParenNode {
      items: Vec::new(),
    }
  }
  pub fn add(&mut self, item: ParenNodeItem) {
    self.items.push(item);
  }
}



pub struct ParenChildrenStack {
  items: Vec<ParenNode>,
}

impl ParenChildrenStack {
  pub fn new() -> ParenChildrenStack {
    ParenChildrenStack {
      items: Vec::new(),
    }
  }
  pub fn push(&mut self, p: ParenNode) {
    self.items.push(p);
  }
  pub fn pop(&mut self) -> Option<ParenNode> {
    self.items.pop()
  }
  pub fn add_to_last(&mut self, p: ParenNodeItem) {
    if let Some(node) = self.items.last_mut() {
      node.add(p)
    }
  }
}


#[derive(Debug)]
pub struct ParenTree {
  pub root: ParenNode
}

impl ParenNode {
  
}