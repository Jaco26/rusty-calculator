pub mod errors;
mod accumulator;

use crate::characters::*;
use errors::{
  ParserError, 
  ParserErrorKind::{
    BadVarName,
    AdjacentOperators,
    EmptyExpression,
  }
};
use accumulator::{Accumulator, AccumNode, AccumNodeItem};

#[derive(Debug)]
pub enum UserInput {
  Assignment {
    left: String,
    right: Vec<AccumNode>,
  },
  Expression {
    text: Vec<AccumNode>
  },
  Empty,
}



impl UserInput {
  pub fn new(txt: String) -> Result<UserInput, errors::ParserError> {
    use UserInput::*;

    let txt = txt.trim().to_string();

    if txt.is_empty() || txt == "=" {
      return Ok(Empty);
    }

    let mut split = txt.split("=");

    let left = split.next();
    let right = split.next();

    if let Some(val) = right {
      let var_name = left.unwrap_or_else(|| "").trim();

      var_name_is_valid(&var_name)?;

      let assignment = Assignment {
        left: String::from(var_name),
        right: parse_expression(val)?
      };

      return Ok(assignment);

    } else {
      return Ok(Expression { text: parse_expression(left.unwrap())? }); // because of logic above, left should always have Some
    }
  }
}

fn parse_expression(expr: &str) -> Result<Vec<AccumNode>, ParserError> {
  let mut accum = Accumulator::new();
  let mut exp_items: Vec<&str> = expr.split("").filter(|&x| !x.is_empty()).collect();

  // fn traverse(mut items: Vec<&str>, mut accum: Accumulator) -> Result<Accumulator, ParserError> {
  for c in exp_items {
    let c = c.to_string();

    match categorize_first_of(&c) {
      Some(char_kind) => match char_kind {
        LeftParen => {
          // if the item to the left is NOT None and it is NOT Math or RightParen,
          // buffer and flush a Multiply AccumNodeItem
          if let Some(prev_accum_node_item) = accum.get_last_added() {
            match prev_accum_node_item.kind() {
              RightParen |
              Alpha |
              Number |
              Dot => {
                accum.flush_buffer();
                accum.add_to_buffer(AccumNodeItem::new("*", Math(Multiply)));
                accum.flush_buffer();
              },
              _ => {},
            };
          };
          accum.flush_buffer();
          accum.add_to_buffer(AccumNodeItem::new(&c, char_kind));
          accum.flush_buffer();
        },
        RightParen => {
          accum.flush_buffer();
          accum.add_to_buffer(AccumNodeItem::new(&c, char_kind));
          accum.flush_buffer();
        },

        Alpha |
        Number |
        Dot  => {
          accum.add_to_buffer(AccumNodeItem::new(&c, char_kind))
        },
      
        Math(math_char) => match math_char {
          // Multiply => {
          //   accum.flush_buffer();
          //   if let Some(prev_accum_node_item) = accum.get_last_added() {
          //     if let Some(prev_char_kind) = categorize_first_of(&prev_accum_node_item.value()) {
          //       if let Math(prev_math_char) = prev_char_kind {
          //         if let Multiply = prev_math_char {
          //           accum.pop_values();
          //           accum.add_to_buffer(AccumNodeItem::new("**", Math(Exponent)));
          //         } else { 
          //           // previous char is a math_char and is NOT mulitply! Bad boy!
          //           return Err(ParserError { kind: Some(AdjacentOperators) });
          //         }
          //       } else { 
          //         // previous char is NOT Math...
          //         accum.add_to_buffer(AccumNodeItem::new(&c, Math(Multiply)));
          //       }
          //     }
          //   } else { // if last is None
          //     accum.add_to_buffer(AccumNodeItem::new(&c, Math(Multiply)));
          //   }
          //   accum.flush_buffer();
          // },
          Multiply => {
            accum.flush_buffer();
            match accum.lookback_char_kind() {
              Some(ck) => match ck {
                Math(mc) => match mc {
                  Multiply => {
                    accum.pop_values();
                    accum.add_to_buffer(AccumNodeItem::new("**", Math(Exponent)));
                  },
                  _ => return Err(ParserError { kind: Some(AdjacentOperators) }),
                },
                _ => {
                  accum.add_to_buffer(AccumNodeItem::new(&c, Math(Multiply)));
                }
              },
              None => {
                accum.add_to_buffer(AccumNodeItem::new(&c, Math(Multiply)));
              },
            };
            accum.flush_buffer();
            // accum.lookback(|this, prev_char_kind| {

            //   if let Math(prev_math_char) = prev_char_kind {
            //     if let Multiply = prev_math_char {
            //       this.pop_values();
            //       this.add_to_buffer(AccumNodeItem::new("**", Exponent));
            //     } else {
            //       // previous char is a math_char and is NOT mulitply! Bad boy!
            //       return Err(ParserError { kind: Some(AdjacentOperators) });
            //     }
                
            //   } else
              
            // });
          },

          Divide |
          Add |
          Subtract => {
            accum.flush_buffer();
            accum.add_to_buffer(AccumNodeItem::new(&c, Math(math_char)));
            accum.flush_buffer();
          },

          _ => {}
        },

        Space => {
          accum.flush_buffer();
        },
      },
      None => {},
    }
  }
  accum.flush_buffer();
  // Ok(accum)
  // };



  // let result = traverse(exp_items, accum)?;

  if let Some(values) = accum.values() {
    Ok(values)
  } else {
    println!("There is nothing in accum._values");
    Err(ParserError { kind: None })
  }
}

fn var_name_is_valid(var_name: &str) -> Result<(), ParserError> {
  match categorize_first_of(var_name) {
    Some(val) => match val {
      Number => return Err(ParserError { kind: Some(BadVarName("variable names cannot start with numbers!")) }),
      
      Math(_) | 
      Dot | 
      LeftParen | 
      RightParen => return Err(ParserError { kind: Some(BadVarName("variable names cannot include any reserved characters")) }),
      
      Alpha | Space => {},
    },
    None => {},
  }

  for c in var_name.split("") {
    match categorize_first_of(c) {
      Some(val) => match val {
        Math(_) | 
        Dot | 
        Space | 
        LeftParen | 
        RightParen => return Err(ParserError { kind: Some(BadVarName("variable names cannot include any reserved characters or spaces")) }),
        
        Alpha | Number  => {},
      },
      None => {},
    }
  }

  Ok(())
}
