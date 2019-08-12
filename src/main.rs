
#[macro_use]
extern crate strum_macros;
extern crate strum;
extern crate error_chain;

// mod token;
// mod lexer;
// mod parser;
mod vm;
mod repl;

use std::io::{stdin, stdout, Write};

fn main () {
    let mut repl = repl::REPL::new();
    repl.run();
}
