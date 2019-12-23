mod enums;
mod parser;

use std::io::{self, Write};

pub fn run() {
  loop {
    let input = get_user_input().unwrap_or_else(|err| {
      eprintln!("[error getting user input] {:?}", err);
      std::process::exit(1);
    });

    println!("You said ({} chars): {}", input.len(), input);
  }
}


fn get_user_input() -> Result<String, io::Error> {
  print!("> ");
  io::stdout().flush()?;
  let mut rv = String::new();
  io::stdin().read_line(&mut rv)?;
  rv.pop();
  Ok(rv.trim().to_string())
}