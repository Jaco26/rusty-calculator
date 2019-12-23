
use super::expression::Expression;

#[derive(Debug)]
pub struct UserInput {
  pub assign_to: Option<String>,
  pub expression: Expression,
}

impl UserInput {
  pub fn new(assign_to: Option<String>, expression: Expression) -> UserInput {
    UserInput {
      assign_to,
      expression,
    }
  }
}