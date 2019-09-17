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

  fn traverse(mut items: Vec<&str>, mut accum: Accumulator) -> Result<Accumulator, ParserError> {
    for c in items {
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
            Multiply => {
              accum.flush_buffer();
              if let Some(prev_accum_node_item) = accum.get_last_added() {
                if let Some(prev_char_kind) = categorize_first_of(&prev_accum_node_item.value()) {
                  if let Math(prev_math_char) = prev_char_kind {
                    if let Multiply = prev_math_char {
                      accum.pop_values();
                      accum.add_to_buffer(AccumNodeItem::new("**", Math(Exponent)));
                    } else { 
                      // previous char is a math_char and is NOT mulitply! Bad boy!
                      return Err(ParserError { kind: Some(AdjacentOperators) });
                    }
                  } else { 
                    // previous char is NOT Math...
                    accum.add_to_buffer(AccumNodeItem::new(&c, Math(Multiply)));
                  }
                }
              } else { // if last is None
                accum.add_to_buffer(AccumNodeItem::new(&c, Math(Multiply)));
              }
              accum.flush_buffer();
            },

            Divide |
            Add |
            Subtract => {
              accum.flush_buffer();
              accum.add_to_buffer(AccumNodeItem::new(&c, Math(math_char)));
              accum.flush_buffer();
            }

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
    Ok(accum)
  };

  let mut exp_items: Vec<&str> = expr.split("").filter(|&x| !x.is_empty()).collect();

  let mut accum = Accumulator::new();

  let result = traverse(exp_items, accum)?;

  if let Some(values) = result.values() {
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


// #![allow(dead_code, unused_imports)]

// use crate::constants::*;
// use super::errors::{
//   ParserError, 
//   ParserErrorKind::{
//     BadVarName,
//     EmptyExpression,
//     AdjacentOperators,
//   }
// };
// use UserInput::{
//   Assignment, 
//   Expression, 
//   Empty
// };



// #[derive(Debug)]
// pub enum UserInput {
//   Assignment {
//     left: String,
//     right: Vec<String>,
//   },
//   Expression {
//     text: Vec<String>,
//   },
//   Empty,
// }


// impl UserInput {
//   pub fn new(txt: &str) -> Result<UserInput, ParserError> {
//     let txt = txt.trim();

//     if txt.is_empty() || txt == "=" {
//       return Ok(Empty);
//     }

//     let mut split = txt.split("=");

//     let left = split.next();
//     let right = split.next();

//     if let Some(val) = right {

//       let var_name = left.unwrap_or_else(|| "").trim();

//       var_name_is_valid(&var_name)?; // because of logic above, left should always have Some
      
//       let assignment = Assignment {
//         left: String::from(var_name),
//         right: parse_expression(val)?,
//       };
//       return Ok(assignment);

//     } else {

//       let expression = Expression {
//         text: parse_expression(left.unwrap())?,
//       };

//       return Ok(expression);
//     }
//   }
// }

// fn operators_include(x: &str) -> bool {
//   OPERATORS.iter().any(|&o| x == o)
// }


// fn parse_expression(expr: &str) -> Result<Vec<String>, ParserError> {
  
//   let mut accumulator = vec![];

//   let no_spaces = expr.replace(" ", "");

//   let mut exp_items: Vec<&str> = no_spaces.split("").filter(|&x| !x.is_empty()).collect();

//   fn traverse_exp_items(mut items: Vec<&str>, mut accum: Vec<String>) -> Result<Vec<String>, ParserError> {
//     let mut section = String::new();

//     while items.len() > 0 {
//       let mut x = items.remove(0).to_string();

//       let last = match accum.last() {
//         Some(val) => val.clone(),
//         None => "".to_string(),
//       };

//       if operators_include(&x) || x == LEFT_PAREN || x == RIGHT_PAREN { // if x in OPERATORS or is PAREN
        
//         if operators_include(&last) && section.len() == 0 { 
//           // we can only be assured the last item in accum is also the item the precedes x from items if section.len() == 0
//           return Err(ParserError { kind: AdjacentOperators });
//         }

//         if section.len() > 0 { // if len(section)
//           accum.push(section);
//         }

//         if x == MULTIPLY && items.len() > 0 && items[0] == MULTIPLY {
//           x.push_str(items.remove(0));
//         }

//         else if x == LEFT_PAREN {
//           if !operators_include(&last) || last == RIGHT_PAREN {
//             accum.push(MULTIPLY.to_string());
//           }
          
//         }

//         accum.push(x);

//         return traverse_exp_items(items, accum);
//       } else { // x not in OPERATORS
//         section.push_str(&x);
//       }
//     }
    
//     if section.len() > 0 {
//       accum.push(section);
//     }

//     Ok(accum)
//   }

//   let parsed = traverse_exp_items(exp_items, accumulator)?;

//   Ok(parsed)
// }


// fn var_name_is_valid(var_name: &str) -> Result<(), ParserError> {
//   let first = var_name.chars().next().unwrap_or_default();

//   if first.is_numeric() {
//     return Err(ParserError { 
//       kind: BadVarName("variable names cannot start with numeric characters") 
//     })
//   }

//   if RESERVED_CHARS.iter().any(|&x| var_name.contains(x)) {
//     return Err(ParserError {
//       kind: BadVarName("variable names cannot contain reserved characters")
//     });
//   }

//   if var_name.contains(SPACE) {
//     return Err(ParserError {
//       kind: BadVarName("variable names cannot include spaces, use underscores instead (ex. my_var = something)")
//     });
//   }

//   Ok(())
// }
