pub mod lang;

use std::{env, fs};
use chumsky::prelude::*;
use crate::lang::lexer::lexer;
use crate::lang::parser::parser;

fn main() {
    let src = fs::read_to_string(
        env::args().nth(1).expect("Filename not provided")
    ).expect("Failed to read file");
    let (tokens, mut errs) = lexer()
        .parse(src.as_str())
        .into_output_errors();

    match tokens {
        Some(toks) => {
            let (ast, mut parse_errors) = parser()
                .map_with(|ast, e| (ast, e.span()))
                .parse(toks.as_slice().spanned((src.len()..src.len()).into()))
                .into_output_errors();

            match ast {
                Some(nodes) => { fs::write("./test.ast", format!("{:?}", nodes)); },
                None => { println!("ParsingErrors: {:?}", parse_errors); },
            }
        }
        None => { println!("TokenizationErrors: {:?}", errs) }
    }
}
