pub mod lang;

use std::{env, fs};
use chumsky::prelude::*;
use crate::lang::lexer::lexer;
use crate::lang::parser::statement::statement;

fn main() {
    // let src = fs::read_to_string(
    //     env::args().nth(1).expect("Filename not provided")
    // ).expect("Failed to read file");
    let src = "switch (5) { case 5+5 : {} case 6 : {} }".to_string();
    let (tokens, mut errs) = lexer()
        .parse(src.as_str())
        .into_output_errors();

    let toks = tokens.unwrap();
    let (ast, mut parse_errors) = statement()
        .map_with(|ast, e| (ast, e.span()))
        .parse(toks.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    match ast {
        Some(exprs) => println!("{:?}", exprs),
        None => println!("{:?}", parse_errors),
    }
}
