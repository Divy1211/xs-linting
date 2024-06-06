pub mod lang;

use std::{env, fs};
use chumsky::prelude::*;
use crate::lang::lexer::lexer;

fn main() {
    let src = fs::read_to_string(
        env::args().nth(1).expect("Filename not provided")
    ).expect("Failed to read file");
    let (tokens, mut errs) = lexer()
        .parse(src.as_str())
        .into_output_errors();

    match tokens {
        Some(tokens) => println!("{:?}", tokens),
        None => println!("F")
    }
}
