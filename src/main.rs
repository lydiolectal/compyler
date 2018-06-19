#[macro_use]
extern crate structopt;

mod lexer;
mod token;
mod error;
mod parser;
mod program;
mod codegen;

use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;
use lexer::Lexer;

#[derive(Debug, StructOpt)]
#[structopt(name = "compyler", about = "The Compyler compiler.")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let text = fs::read_to_string(opt.input)
        .expect("Failed to read input.");
    let lexer = Lexer::new(&text);
    let tokens = lexer.lex()
        .expect("Failed to lex input.");
    println!("{:?}", tokens);
}
