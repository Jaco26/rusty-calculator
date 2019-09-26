use super::{
  accumulator::{Accumulator, ExpressionNode, ExpressionNodeItem},
  errors::{self, ParserError, ParserErrorKind::*},
};
use crate::characters::*;

#[derive(Debug, Clone)]
pub struct Expression {
  _nodes: Vec<ExpressionNode>,
  _idx: usize,
}
// pub struct Expression(Vec<ExpressionNode>);

impl Expression {
  pub fn new(exp: &str) -> Result<Expression, errors::ParserError> {
    let mut accum = Accumulator::new();
    let exp_items: Vec<&str> = exp.split("").filter(|&x| !x.is_empty()).collect();

    for (i, c) in exp_items.iter().enumerate() {
      let c = c.to_string();

      let thing = categorize_first_of(&c);

      if i == exp_items.len() - 1 {
        match thing {
          Some(Math(_)) => return Err(ParserError { kind: Some(OperatorEndsScope) }),
          Some(_) | None => {},
        };
      } else if i == 0 {
        match thing {
          Some(Math(_)) => return Err(ParserError { kind: Some(OperatorBeginsScope) }),
          Some(_) | None => {},
        };
      }
      
      match categorize_first_of(&c) {
        Some(char_kind) => match char_kind {
          LeftParen => {
            // if the item to the left is NOT None and it is NOT Math or RightParen,
            // buffer and flush a Multiply ExpressionNodeItem
            if let Some(prev_accum_node_item) = accum.lookback_char_kind() {
              match prev_accum_node_item {
                RightParen | Alpha | Number | Dot => {
                  accum.add_item(ExpressionNodeItem::new("*", Math(Multiply)));
                },
                Math(_) => return Err(ParserError { kind: Some(OperatorBeginsScope) }),
                _ => {},
              };
            };
            accum.add_item(ExpressionNodeItem::new(&c, char_kind));
          },
          RightParen => {
            match accum.lookback_char_kind() {
              Some(Math(_)) => return Err(ParserError { kind: Some(OperatorEndsScope)}),
              Some(LeftParen) => return Err(ParserError { kind: Some(EmptyParens) }),
              Some(_) | None => {},
            }
            accum.add_item(ExpressionNodeItem::new(&c, char_kind));
          },

          Alpha | Number | Dot  => {
            accum.add_to_buffer(ExpressionNodeItem::new(&c, char_kind))
          },
        
          Math(math_char) => match math_char {
            Multiply => match accum.lookback_char_kind() {
              Some(Math(Multiply)) => {
                accum.pop_values();
                accum.add_item(ExpressionNodeItem::new("**", Math(Exponent)));
              },
              Some(Math(_)) => return Err(ParserError { kind: Some(AdjacentOperators) }),
              Some(_) | None => accum.add_item(ExpressionNodeItem::new(&c, Math(Multiply))),
            },

            Divide | Add | Subtract => {
              accum.add_item(ExpressionNodeItem::new(&c, Math(math_char)));
            },
            _ => {}
          },
          Space => {
            accum.flush_buffer();
          },
          Other => {},
        },
        None => {},
      }
    }
    
    accum.flush_buffer();

    if !accum.parens_are_balanced() {
      return Err(ParserError { kind: Some(UnbalancedParens) })
    }

    if let Some(values) = accum.values() {
      Ok(Expression {
        _nodes: values,
        _idx: 0,
      })
    } else {
      println!("There is nothing in accum._values");
      Err(ParserError { kind: None })
    }
  } // end Expression::new
}


impl Iterator for Expression {
  type Item = ExpressionNode;

  fn next(&mut self) -> Option<Self::Item> {
    if self._idx < self._nodes.len() {
      let x = self._nodes[self._idx].clone();
      self._idx += 1;
      return Some(x);
    }
    None
  }
}