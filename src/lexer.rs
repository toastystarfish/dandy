
use crate::token::Token;
use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'lex> {
  chars: Peekable<Chars<'lex>>
}

impl<'lex> Lexer<'lex> {
  pub fn new(s: &'lex mut String) -> Self {
    Self {
      chars: s.chars().peekable()
    }
  }

  fn get_integer(&mut self, first: char) -> Option<Token> {
    let mut int_string = first.to_string();

    while let Some(&c) = self.chars.peek() {
      if !c.is_digit(10) { break; }
      int_string.push(c);
      self.chars.next();
    }

    if let Ok(i) = int_string.parse::<i64>() {
      Some(Token::Integer(i))
    } else {
      None
    }
  }
}

impl<'lex> Iterator for Lexer<'lex> {
  type Item = Token;

  fn next(&mut self) -> Option<Self::Item> {
    match self.chars.next() {
      Some('+') => Some(Token::Plus),
      Some('-') => Some(Token::Minus),
      Some(i) if i.is_digit(10) => self.get_integer(i),
      Some(c) if c.is_whitespace() => self.next(),
      Some(c) => panic!(format!("Unrecognized token: {}", c)),
      None => None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
}