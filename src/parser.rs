
use std::collections::VecDeque;
use crate::token::Token;

pub struct Parser {
  token_queue: VecDeque<Token>
}

impl Parser {
  pub fn new() -> Self {
    Self {
      token_queue: VecDeque::<Token>::new()
    }
  }

  pub fn add_to_queue(&mut self, vec: Vec<Token>) {
    let mut as_queue: VecDeque<Token> = vec.into_iter().collect();
    self.token_queue.append(&mut as_queue);
  }
}