use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use chumsky::input::Input;
use chumsky::Parser;

use crate::lint::gen_errs::{gen_errs, gen_xs_errs};
use crate::parsing::lexer::lexer;
use crate::parsing::lexer::token::Token;
use crate::parsing::parser::parser;
use crate::r#static::type_check::{LocalEnv, TypeEnv};
use crate::r#static::type_check::statements::xs_tc;
use crate::r#static::xs_error::{XSError};

pub fn gen_info_from_path(
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut HashSet<String>,
    path: PathBuf,
    ignores: &HashSet<u32>,
) {
    let src = match fs::read_to_string(&path) {
        Ok(src) => {src}
        Err(err) => {
            let path = path.to_str().unwrap();
            eprintln!("Failed to read path {path}, details: {err}");
            return;
        }
    };
    let filename = path.to_str().unwrap();
    
    let errs = gen_info_from_src(
        type_env, local_envs, groups,
        &path, &src, ignores
    );

    gen_xs_errs(&errs, filename, &src, ignores);
    
    if errs.len() == 0 {
        println!("No errors found in file '{filename}'! Your code is free of the pitfalls of XS' quirks =)");
    }
    println!("Finished analysing '{filename}'.")
}

pub fn gen_info_from_src(
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut HashSet<String>,
    path: &PathBuf,
    src: &str,
    ignores: &HashSet<u32>,
) -> Vec<XSError> {
    let (tokens, errs) = lexer()
        .parse(src)
        .into_output_errors();

    let mut tc_errs = vec![];
    
    let Some(mut tokens) = tokens else {
        gen_errs("TokenizationError", &errs, path, src);
        return tc_errs;
    };
    
    tokens = tokens.into_iter()
        .filter(|tok| match tok { (Token::Comment(_), _) => { false }, _ => { true } })
        .collect();

    let (ast, errs) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        gen_errs("ParsingError", &errs, path, src);
        return tc_errs;
    };
    
    xs_tc(path, &ast, &mut None, type_env, local_envs, groups, &mut tc_errs, ignores);
    
    tc_errs
}