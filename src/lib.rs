mod characters;
mod enums;
mod parser;
mod evaluator;

use std::collections::HashMap;
use parser::{UserInput, UserInputParser};

pub fn run() {
  let mut parser = UserInputParser::new();

  let mut history = Vec::new();

  let mut evaluated: HashMap<String, f64> = HashMap::new();

  loop {
    parser.get_input()
      .expect("Error getting input from terminal");

    history.push(parser.text_copy());

    let mut user_input = parser.parse().unwrap_or_else(|err| {
      eprintln!("{:?}", err);
      None
    });

    println!("UserInput: {:#?}", user_input);

    if let Some(UserInput { assign_to, expression }) = user_input {
      let evaluation_result = evaluator::evaluate(expression, &evaluated).unwrap_or_else(|err| {
        println!("{:?}", err);
        0.0
      });
      if let Some(name) = assign_to {
        &evaluated.insert(name, evaluation_result);
      }
    }

  }
}