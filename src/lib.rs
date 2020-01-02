#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

mod enums;
mod errors;
mod parser;
mod evaluator;

use std::io::{self, Write};
use std::collections::HashMap;


pub fn run() {

  let mut evaluated: HashMap<String, f64> = HashMap::new();

  loop {
    let mut raw_input = get_user_input().unwrap_or_else(|err| {
      eprintln!("[error getting user input] {:?}", err);
      std::process::exit(1);
    });

    let parser_result = parser::parse(&mut raw_input).unwrap_or_else(|err| {
      eprintln!("{:#?}", err);
      None
    });


    if let Some(parser_result) = parser_result {
      match parser_result.expression {
        Some(exp_tree) => {
          if let Some(root) = exp_tree.root {
            let evaluation_result = evaluator::evaluate(&evaluated, root).unwrap_or_else(|err| {
              eprintln!("{:#?}", err);
              None
            });
            if let Some(value) = evaluation_result {
              match parser_result.assign_to {
                Some(variable_name) => {
                  evaluated.insert(variable_name, value);
                },
                None => {
                  println!("{}", value);
                },
              }
            }
          }
        },
        None => {
          if let Some(variable_name) = parser_result.assign_to {
            if let Some(stored_value) = evaluated.get(&variable_name) {
              println!("{}", stored_value);
            }
          }
        },
      }
    }
  }
}

/// Get text input from the command line
fn get_user_input() -> Result<String, io::Error> {
  print!("> ");
  io::stdout().flush()?;
  let mut rv = String::new();
  io::stdin().read_line(&mut rv)?;
  rv.pop();
  Ok(rv.trim().to_string())
}