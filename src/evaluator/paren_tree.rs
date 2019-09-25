
// from crate
use crate::parser::Expression;
use crate::enums::{
  CharKind,
  Operator,
  EvaluationItem,
};
// from parent mod
use super::errors::EvaluationError;

struct ParenNode {
  items: Vec<EvaluationItem>,
}

impl ParenNode {
  pub fn new() -> ParenNode {
    ParenNode {
      items: Vec::new(),
    }
  }
}

pub struct ParenTree {
  root: ParenNode
}

impl ParenTree {
  // pub fn from(expression: Expression) -> Result<ParenTree, EvaluationError> {
    
  //   fn traverse_expression(expr: Expression) {
  //     let node = ParenNode::new();

  //     if let Some(exp_node) = expr.pop() {
  //       if let Some(contents) = exp_node.contents() {

  //         if contents.iter().all(|x| match x.kind() { Operator => true }) { // if all contents are Operator
  //           // node.items.push(EvaluationItem::Math());
  //         } else if true {

  //         }

  //       }
  //     }
  //   }

  // }
}

// fn categorize_expression_node

//  def parse_char_list(self, char_list: list):
//     def traverse_list(lst: list):
//       node = ParenNode()
//       while len(lst):
//         char = lst.pop(0)
//         if char == LEFT_PAREN:
//           paren_node_item = traverse_list(lst)
//           node.items.append(paren_node_item)
//         elif char == RIGHT_PAREN:
//           return node
//         else:
//           node.items.append(char)
//       return node
    
//     self.root = traverse_list(char_list)
  