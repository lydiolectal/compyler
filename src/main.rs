#[cfg(test)]
extern crate tempfile;
#[cfg(test)]
extern crate regex;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod lexer;
mod token;
mod error;
mod parser;
mod program;
mod codegen;
mod wexp;
mod compile;

use std::io::{stdin, Read};

fn main() {
    let mut text = String::new();
    stdin().read_to_string(&mut text).expect("Failed to read input.");
    let wexp = compile::compile(&text)
        .expect("Compilation failed.");
    println!("{}", wexp);
}
