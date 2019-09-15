use std::io::{self, Write};

pub fn get_input() -> Result<String, io::Error> {
  print!("> ");
  io::stdout().flush()?;
  let mut ret = String::new();
  io::stdin().read_line(&mut ret)?;
  ret.pop();
  Ok(ret)
}

