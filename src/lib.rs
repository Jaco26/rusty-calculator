#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

mod structs;
mod enums;
mod errors;
mod parser;

use std::io::{self, Write};
use std::collections::HashMap;

use parser::UserInputParser;


pub fn run() {

  let mut evaluated: HashMap<String, f64> = HashMap::new();

  loop {
    let raw_input = get_user_input().unwrap_or_else(|err| {
      eprintln!("[error getting user input] {:?}", err);
      std::process::exit(1);
    });

    let mut parser = UserInputParser::new();

    let parsed_input = parser.parse(&raw_input).unwrap_or_else(|err| {
      eprintln!("{:#?}", err);
      None
    });

    if let Some(tree) = parsed_input {
      println!("{:#?}", tree);
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