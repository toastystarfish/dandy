
use std::fmt;

#[derive(Debug, PartialEq, EnumDiscriminants)]
pub enum Token {
  Integer(i64),
  Minus,
  Plus
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Token::Integer(i) => write!(f, "{}", i),
      Token::Minus => write!(f, "-"),
      Token::Plus => write!(f, "+")
    }
  }
}

impl Token {
  pub fn is_type(&self, expected: TokenDiscriminants) -> bool {
    let actual: TokenDiscriminants = self.into();
    actual == expected
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_checking_token_types() {
    assert!(Token::Integer(32).is_type(TokenDiscriminants::Integer));
  }
}