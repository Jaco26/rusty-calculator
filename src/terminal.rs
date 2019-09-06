use std::{io::{self, Write}};

pub fn get_input(prompt: &str) -> Result<String, io::Error> {
  print!("{}\n> ", prompt);
  io::stdout().flush().unwrap();
  let mut ret = String::new();
  io::stdin().read_line(&mut ret)?;
  ret.pop();
  Ok(ret)
}
