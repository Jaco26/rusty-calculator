mod characters;
mod parser;
mod evaluator;

use std::collections::HashMap;
use parser::UserInputParser;

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

  }
}