#![allow(dead_code, unused_imports)]

use super::errors::{ParserError, ParserErrorKind};
use UserInput::{Expression, Statement, Empty};


#[derive(Debug)]
pub enum UserInput {
  Statement {
    left: String,
    right: String,
  },
  Expression {
    text: String,
  },
  Empty,
}


impl UserInput {
  pub fn new(txt: &str) -> Result<UserInput, Box<dyn std::error::Error>> {
    let txt = txt.trim();

    if txt.is_empty() || txt == "=" {
      return Ok(Empty);
    }

    let mut split = txt.split("=");

    let left = split.next();
    let right = split.next();

    if let Some(val) = right {
      return Ok(
        Statement {
          left: String::from(left.unwrap_or("").trim()),
          right: String::from(val.trim()),
        }
      );
    } else {
      return Ok(
        Expression {
          text: String::from(left.unwrap()),
        }
      );
    }
  }


  pub fn is_valid(&self) -> Result<(), ParserError> {
    match self {
      Statement { left, right } => {
        validate::var_name(left)?;
        validate::expression(right)?;
        Ok(())
      },
      Expression { text } => {
        validate::expression(text)?;
        Ok(())
      },
      UserInput::Empty => Ok(()),
    }
  }
}


mod validate {
  use crate::constants::*;
  use super::{ParserError, ParserErrorKind::{BadVarName, EmptyExpression}};


  pub fn expression(expr: &str) -> Result<(), ParserError> {
    if expr.is_empty() {
      return Err(ParserError{ kind: EmptyExpression });
    }
  
    Ok(())
  }


  pub fn var_name(var_name: &str) -> Result<(), ParserError> {
    let first = var_name.chars().next().unwrap_or_default();

    if first.is_numeric() {
      return Err(ParserError { 
        kind: BadVarName("variable names cannot start with numeric characters") 
      })
    }

    if RESERVED_CHARS.iter().any(|&x| var_name.contains(x)) {
      return Err(ParserError {
        kind: BadVarName("variable names cannot contain reserved characters")
      });
    }

    if var_name.contains(SPACE) {
      return Err(ParserError {
        kind: BadVarName("variable names cannot include spaces, use underscores instead (ex. my_var = something)")
      });
    }

    Ok(())
  }
}



