pub mod expression_tree;

use crate::enums::{CharKind, MathOperator, ExpressionNodeKind};
use crate::errors::SyntaxError;

use expression_tree::ExpressionTree;

pub fn parse(input: &str) -> Result<Option<ExpressionTree>, SyntaxError> {

  let mut accum = ExpressionAccumulator::new();

  for (i, ch) in input.char_indices() {
    match categorize_char(ch) {

      CharKind::Alpha => {
        accum.buffer.set_kind(ExpressionNodeKind::VariableName);
        accum.buffer.value.push(ch);
        if i == input.len() - 1 {
          accum.flush_buffer();
        }
      },

      CharKind::Number |
      CharKind::Dot => {
        accum.buffer.set_kind(ExpressionNodeKind::Float);
        accum.buffer.value.push(ch);
        if i == input.len() - 1 {
          accum.flush_buffer();
        }
      },

      CharKind::LeftParen => {
        accum.flush_buffer();
        accum.buffer.set_kind(ExpressionNodeKind::LeftParen);
        accum.buffer.value.push(ch);
        accum.flush_buffer();
      },

      CharKind::RightParen => {
        accum.flush_buffer();
        accum.buffer.set_kind(ExpressionNodeKind::RightParen);
        accum.buffer.value.push(ch);
        accum.flush_buffer();
      },

      CharKind::Operator(math_op) => {
        match math_op {
          MathOperator::Multiply => {
            if let Some(last_item) = accum.items.last() {
              if let ExpressionNodeKind::Operator(MathOperator::Multiply) = last_item.kind {
                accum.buffer.set_kind(ExpressionNodeKind::Operator(MathOperator::Exponent));
                accum.buffer.value.push_str("**");
                accum.items.pop();
                accum.flush_buffer();
              } else {
                accum.flush_buffer();
                accum.buffer.set_kind(ExpressionNodeKind::Operator(math_op));
                accum.buffer.value.push(ch);
                accum.flush_buffer();
              }
            } else {
              accum.flush_buffer();
              accum.buffer.set_kind(ExpressionNodeKind::Operator(math_op));
              accum.buffer.value.push(ch);
              accum.flush_buffer();
            }
          },
          _ => {
            accum.flush_buffer();
            accum.buffer.set_kind(ExpressionNodeKind::Operator(math_op));
            accum.buffer.value.push(ch);
            accum.flush_buffer();
          }
        }
      },

      CharKind::Space => {
        accum.flush_buffer();
        accum.buffer.set_kind(ExpressionNodeKind::Space);
        accum.buffer.value.push(ch);
        accum.flush_buffer();
      },

      _ => {
        eprintln!("[warning] unknown character: '{}'", ch);
      }
    }
  }

  let expression_nodes: Vec<ExpressionNode> = accum.items.iter().fold(Vec::new(), |mut acc, x| {
    match x.kind {
      ExpressionNodeKind::Init |
      ExpressionNodeKind::Space => {},

      _ => {
        acc.push(x.clone());
      },
    }

    acc
  });

  let mut tree = ExpressionTree::new();

  tree.parse(expression_nodes)?;

  Ok(Some(tree))
}


fn categorize_char(c: char) -> CharKind {
  match c {
    ' ' => CharKind::Space,
    '(' => CharKind::LeftParen,
    ')' => CharKind::RightParen,
    '.' => CharKind::Dot,
    '-' => CharKind::Operator(MathOperator::Subtract),
    '+' => CharKind::Operator(MathOperator::Add),
    '/' => CharKind::Operator(MathOperator::Divide),
    '*' => CharKind::Operator(MathOperator::Multiply),
    '0'..='9' => CharKind::Number,

    'a'..='z' |
    'A'..='Z' |
    '_' => CharKind::Alpha,

    _ => CharKind::Other,
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionNode {
  pub kind: ExpressionNodeKind,
  pub value: String,
}

impl ExpressionNode {
  fn new() ->  ExpressionNode{
    ExpressionNode {
      kind: ExpressionNodeKind::Init,
      value: String::new(),
    }
  }
  fn set_kind(&mut self, t: ExpressionNodeKind) {
    if let ExpressionNodeKind::Init = self.kind {
      self.kind = t;
    }
  }
}


#[derive(Debug, Clone)]
struct ExpressionAccumulator {
  buffer: ExpressionNode,
  items: Vec<ExpressionNode>,
}

impl ExpressionAccumulator {
  fn new() -> ExpressionAccumulator {
    ExpressionAccumulator {
      buffer: ExpressionNode::new(),
      items: Vec::new(),
    }
  }

  fn flush_buffer(&mut self) {
    self.items.push(self.buffer.clone());
    self.buffer = ExpressionNode::new();
  }
}



