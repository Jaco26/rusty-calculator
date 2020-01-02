pub mod expression_tree;

use crate::enums::{CharKind, MathOperator, ExpressionNodeKind};
use crate::errors::SyntaxError;

use expression_tree::ExpressionTree;

pub fn parse(input: &mut str) -> Result<Option<ParserResult>, SyntaxError> {

  let mut var: Option<&str> = None;

  let mut exp: Option<&str> = None;

  if let Some(i) = input.find('=') {
    let split_input = input.split_at(i + 1);
    var = Some(split_input.0);
    exp = Some(split_input.1);
  } else if let None = exp {
    exp = Some(input);
  }
  
  let mut rv = ParserResult::new();

  if let Some(variable_name) = var {
    let mut variable_name = variable_name.to_string();
    variable_name.pop();
    variable_name = variable_name.trim().to_string();
    rv.assign_to = Some(variable_name);
  }

  let mut accum = ExpressionAccumulator::new();

  if let Some(expression) = exp {
    for (i, ch) in expression.to_string().char_indices() {
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
  
        CharKind::AssignmentOperator => {},
  
        _ => {
          eprintln!("[warning] unknown character: '{}'", ch);
        }
      }
    }
  }

  accum.flush_buffer();

  let mut expression_nodes: Vec<ExpressionNode> = accum.items.iter().fold(Vec::new(), |mut acc, x| {
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

  rv.expression = Some(tree);

  Ok(Some(rv))
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

    '=' => CharKind::AssignmentOperator,

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


#[derive(Debug)]
pub struct ParserResult {
  pub expression: Option<ExpressionTree>,
  pub assign_to: Option<String>,
}

impl ParserResult {
  fn new() -> ParserResult {
    ParserResult { expression: None, assign_to: None }
  }
}