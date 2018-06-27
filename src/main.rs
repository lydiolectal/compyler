#[cfg(test)]
extern crate regex;
#[cfg(test)]
extern crate tempfile;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod codegen;
mod compile;
mod error;
mod lexer;
mod parser;
mod program;
mod token;
mod wexp;

use std::io::{stdin, Read};

fn main() {
    let mut text = String::new();
    stdin()
        .read_to_string(&mut text)
        .expect("Failed to read input.");
    let wexp = compile::compile(&text).expect("Compilation failed.");
    println!("{}", wexp);
}
