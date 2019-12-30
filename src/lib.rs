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
    let raw_input = get_user_input().unwrap_or_else(|err| {
      eprintln!("[error getting user input] {:?}", err);
      std::process::exit(1);
    });

    let parse_result = parser::parse(&raw_input).unwrap_or_else(|err| {
      eprintln!("{:#?}", err);
      None
    });

    if let Some(tree) = parse_result {
      if let Some(root) = tree.root {
        let result = evaluator::evaluate(root).unwrap_or_else(|err| {
          eprintln!("{:#?}", err);
          None
        });

        if let Some(result) = result {
          println!("Result: {}", result);
        }
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