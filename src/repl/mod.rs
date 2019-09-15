#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

mod terminal;
mod parser;
mod errors;

use std::collections::HashMap;
use parser::UserInput;
use terminal::get_input;
use crate::evaluator::evaluate_expression;



pub fn run() {
  let mut history = Vec::new();

  let mut evaluated_input_map: HashMap<String, f64> = HashMap::new();

  loop {
    let raw_input = get_input().unwrap();

    history.push(raw_input.clone()); // raw_input is dropped at the end of the loop scope

    let user_input = UserInput::new(&raw_input).unwrap();

    match user_input.is_valid() {
      Ok(()) => {
        match user_input {
          UserInput::Statement { left, right } => {
            match evaluate_expression(&right, &evaluated_input_map) {
              Ok(val) => {
                &evaluated_input_map.insert(left, val);
              },
              Err(err) => eprintln!("{}", err),
            };
          },
          UserInput::Expression { text } => {
            match evaluate_expression(&text, &evaluated_input_map) {
              Ok(val) => println!("{} = {}", text, val),
              Err(err) => eprintln!("{}", err),
            };
          },
          UserInput::Empty => {},
        };
      },
      Err(err) => eprintln!("{}", err),
    };
  }
}