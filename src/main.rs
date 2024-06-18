pub mod parsing;
pub mod r#static;

use std::{env, fs};
use std::collections::{HashMap, HashSet};
use chumsky::prelude::*;
use crate::parsing::lexer::lexer;
use crate::parsing::parser::parser;
use crate::r#static::type_check::statements::xs_tc;

fn main() {
    let src = fs::read_to_string(
        env::args().nth(1).expect("Filename not provided")
    ).expect("Failed to read file");
    
    let (tokens, errs) = lexer()
        .parse(src.as_str())
        .into_output_errors();
    
    let Some(tokens) = tokens else {
        println!("TokenizationErrors: {:?}", errs);
        return;
    };

    let (ast, parse_errors) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        println!("ParsingErrors: {:?}", parse_errors);
        return;
    };
    
    let mut type_env = HashMap::new();
    let mut groups = HashSet::new();
    let mut errs = vec![];

    xs_tc(&ast, &mut type_env, &mut groups, &mut errs);
    println!("TypeEnv: {:?}", type_env);
    println!("Errors: {:?}", errs);
}
