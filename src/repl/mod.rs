#![allow(dead_code, unused_imports, unused_mut, unused_variables)]

mod terminal;
pub mod parser;

use std::collections::HashMap;
use parser::errors;
use parser::UserInput::{
  self,
  Assignment,
  Expression,
  Empty,
};
use terminal::get_input;
use crate::evaluator::evaluate_expression;



pub fn run() {
  let mut history = Vec::new();

  let mut evaluated_input_map: HashMap<String, f64> = HashMap::new();

  loop {
    let raw_input = get_input().unwrap();

    history.push(raw_input.clone()); // raw_input is dropped at the end of the loop scope

    let mut user_input = UserInput::new(raw_input).unwrap_or_else(|err| {
      eprintln!("{:?}", err);
      UserInput::Empty
    });

    println!("UserInput: {:#?}", user_input);

    // match user_input {
    //   Assignment { left, right } => {

    //     match evaluate_expression(right, &evaluated_input_map) {
    //       Ok(result) => {
    //         &evaluated_input_map.insert(left, result);
    //       },
    //       Err(err) => eprintln!("{}", err),
    //     };
        
    //   },
    //   Expression { text } => {

    //     match evaluate_expression(text.clone(), &evaluated_input_map) {
    //       Ok(result) => println!("{} = {}", &raw_input, result),
    //       Err(err) => eprintln!("{}", err),
    //     };

    //   },
    //   Empty => {},
    // }
  }
}