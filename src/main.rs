pub mod parsing;
pub mod r#static;

use std::{env, fs};
use std::collections::HashMap;
use chumsky::prelude::*;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::type_::Type;
use crate::parsing::lexer::lexer;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::statement::statement;
use crate::r#static::type_check::expression::xs_tc_expr;
use crate::r#static::type_check::statement::xs_tc_stmt;
// use crate::parsing::parser::parser;

fn main() {
    // let src = fs::read_to_string(
    //     env::args().nth(1).expect("Filename not provided")
    // ).expect("Failed to read file");

    let src = "float fn(int a = 1, float b = 1.1) {}".to_string();
    
    let (tokens, errs) = lexer()
        .parse(src.as_str())
        .into_output_errors();
    
    let Some(tokens) = tokens else {
        println!("TokenizationErrors: {:?}", errs);
        return;
    };

    let (ast, parse_errors) = statement()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        println!("ParsingErrors: {:?}", parse_errors);
        return;
    };
    
    let mut type_env = HashMap::from([
        (Identifier::new("fn"), Type::Func { is_mutable: true, type_sign: vec![Type::Int, Type::Float, Type::Float] })
    ]);
    let mut errs = vec![];

    // let Some(type_) = xs_tc_expr(&ast, &type_env, &mut errs) else {
    //     println!("Errors: {:?}", errs);
    //     return;
    // };

    xs_tc_stmt(&ast, &mut type_env, &mut errs, true);
    // println!("Type: {:?}", type_);
    println!("TypeEnv: {:?}", type_env);
    println!("Errors: {:?}", errs);

    // fs::write("./test.ast", format!("{:?}", ast)).expect("Unabled to write AST to file");
}
