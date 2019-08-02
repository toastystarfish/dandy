
#[macro_use]
extern crate strum_macros;
extern crate strum;
extern crate error_chain;

mod token;
mod lexer;
mod parser;

use std::io::{stdin, stdout, Write};

fn main () {
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("did not enter a string");
    }
}
