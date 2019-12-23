
use crate::structs::{ExpressionChar, ExpressionNode, ExpressionTree};
use crate::enums::{MathOperator, CharKind, ExpressionNodeKind};
use crate::errors::SyntaxError;

pub struct UserInputParser {
  /// This contains the CharKind and unicode value of each character from the
  /// raw user input. These will later be joined together appropriately to form
  /// ExpressionNodes (e.g. "5.43" from "5", ".", "4", "3")
  expression_chars: Vec<ExpressionChar>,
  /// This contains nodes that will be used during evaluation.
  expression_nodes: Vec<ExpressionNode>,
}

impl UserInputParser {
  pub fn new() -> UserInputParser {
    UserInputParser {
      expression_chars: Vec::new(),
      expression_nodes: Vec::new(),
    }
  }

  fn categorize_char(&self, c: char) -> CharKind {
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

  fn add_exp_node(&mut self, n: ExpressionNode) {
    self.expression_nodes.push(n);
  }

  fn add_exp_char(&mut self, c: ExpressionChar) {
    self.expression_chars.push(c);
  }

  pub fn parse(&mut self, input: &str) -> Result<Option<Vec<ExpressionChar>>, SyntaxError> {
    
    for c in input.chars() {
      self.add_exp_char(ExpressionChar::new(
        self.categorize_char(c),
        c.to_string()
      ));
    }

    Ok(Some(self.expression_chars.clone()))
  }
}
