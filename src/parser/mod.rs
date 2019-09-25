// from std
use std::io::{self, Write};
// from crate
use crate::characters::*;
// child mods
mod errors;
mod expression;
mod user_input;
mod accumulator;
// from child mods
use errors::{ParserError, ParserErrorKind::*};
pub use expression::Expression;
pub use user_input::UserInput;


pub struct UserInputParser {
  _text: String,
}

impl UserInputParser {
  pub fn new() -> UserInputParser {
    UserInputParser {
      _text: String::new(),
    }
  }

  pub fn text_copy(&self) -> String {
    self._text.to_string()
  }

  pub fn get_input(&mut self) -> Result<(), io::Error> {
    print!("> ");
    io::stdout().flush()?;
    let mut ret = String::new();
    io::stdin().read_line(&mut ret)?;
    ret.pop();
    self._text = ret;
    Ok(())
  }

  pub fn parse(&self) -> Result<Option<UserInput>, errors::ParserError> {
    let txt = self._text.trim();
    
    if txt.is_empty() || txt == "=" {
      return Ok(None);
    }

    let mut split = txt.split("=");

    let left = split.next();
    let right = split.next();

    if let Some(exp_text) = right {
      let assign_to = left.unwrap_or_else(|| "")
        .trim()
        .to_string();

      self.var_name_is_valid(&assign_to)?;

      let expression = Expression::new(exp_text)?;

      return Ok(Some(UserInput::new(Some(assign_to), expression)));
    } else {
      let exp_text = left.unwrap_or_else(|| "").trim();

      let expression = Expression::new(exp_text)?;

      return Ok(Some(UserInput::new(None, expression)));
    }
  } // end parse


  fn var_name_is_valid(&self, var_name: &str) -> Result<(), ParserError> {
    match categorize_first_of(var_name) {
      Some(val) => match val {
        Number => return Err(ParserError { kind: Some(BadVarName("variable names cannot start with numbers!")) }),
        
        Math(_) | 
        Dot | 
        LeftParen | 
        RightParen => return Err(ParserError { kind: Some(BadVarName("variable names cannot include any reserved characters")) }),
        
        Alpha | Space | Other => {},
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
          
          Alpha | Number | Other  => {},
        },
        None => {},
      }
    }

    Ok(())
  } // end var_name_is_valid
}



