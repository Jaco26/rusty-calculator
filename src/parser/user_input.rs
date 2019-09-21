
use super::expression::Expression;

#[derive(Debug)]
pub struct UserInput {
  assign_to: Option<String>,
  expression: Expression,
}

impl UserInput {
  pub fn new(assign_to: Option<String>, expression: Expression) -> UserInput {
    UserInput {
      assign_to,
      expression,
    }
  }
}