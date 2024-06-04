pub mod lang;

use chumsky::prelude::*;
use crate::lang::lexer::lexer;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let tokens = lexer().parse(src);

    match tokens {
        Ok(tokens) => println!("{:?}", tokens),
        Err(errors) => {
            for error in errors {
                println!("Error: {:?}", error);
            }
        }
    }
}
