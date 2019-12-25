
use crate::enums::{CharKind, MathOperator, ExpressionNodeKind};
use crate::errors::SyntaxError;


pub fn parse(input: &str) {

  let mut accum = Accum::new();

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
              if let Some(ExpressionNodeKind::Operator(MathOperator::Multiply)) = last_item.kind {
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

  let result: Vec<AccumNode> = accum.items.iter().fold(Vec::new(), |mut acc, x| {
    if let Some(kind) = x.kind.clone() {
      if kind != ExpressionNodeKind::Space {
        acc.push(x.clone());
      }
    }
    acc
  });
  println!("{:#?}", result);
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

#[derive(Debug, Clone)]
struct AccumNode {
  kind: Option<ExpressionNodeKind>,
  value: String,
}

impl AccumNode {
  fn new() ->  AccumNode{
    AccumNode {
      kind: None,
      value: String::new(),
    }
  }
  fn set_kind(&mut self, t: ExpressionNodeKind) {
    if let None = self.kind {
      self.kind = Some(t);
    }
  }
}


#[derive(Debug, Clone)]
struct Accum {
  buffer: AccumNode,
  items: Vec<AccumNode>
}

impl Accum {
  fn new() -> Accum {
    Accum {
      buffer: AccumNode::new(),
      items: Vec::new(),
    }
  }

  fn flush_buffer(&mut self) {
    self.items.push(self.buffer.clone());
    self.buffer = AccumNode::new();
  }
}