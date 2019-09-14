#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

mod terminal;
mod parser;
mod errors;

use std::collections::HashMap;
use parser::UserInput;
use terminal::{input, out, out_err};


pub fn run() {
  let mut map: HashMap<String, String> = HashMap::new();

  loop {
    let raw_input = input().unwrap();

    let user_input = UserInput::new(&raw_input).unwrap();

    match user_input.is_valid() {
      Ok(()) => {
        match user_input {
          UserInput::Statement { left, right } => {
            println!("Evaluating statement: {} = {}", left, right);
          },
          UserInput::Expression { text } => {
            println!("Evaluating expression: {}", text);
          },
          UserInput::Empty => {},
        };
      },
      Err(err) => {
        out_err(err);
      },
    };
  }
}