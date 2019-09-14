use std::io::{self, Write};

pub fn input() -> Result<String, io::Error> {
  print!("> ");
  io::stdout().flush()?;
  let mut ret = String::new();
  io::stdin().read_line(&mut ret)?;
  ret.pop();
  Ok(ret)
}

pub fn out(text: &str) {
  println!(">> {}", text);
}

pub fn out_err(err: impl std::error::Error) {
  eprintln!("{:?}", err);
}