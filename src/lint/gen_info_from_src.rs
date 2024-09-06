use std::collections::{HashSet};
use std::path::PathBuf;
use chumsky::prelude::*;

use crate::parsing::lexer::lexer;
use crate::parsing::parser::parser;
use crate::r#static::type_check::{LocalEnv, TypeEnv};
use crate::r#static::type_check::statements::xs_tc;
use crate::r#static::xs_error::XSError;

pub fn gen_info_from_src(
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut HashSet<String>,
    path: &PathBuf,
    src: &String
) -> Vec<XSError> {
    let (tokens, errs) = lexer()
        .parse(src.as_str())
        .into_output_errors();

    let mut tc_errs = vec![];
    
    let Some(tokens) = tokens else {
        println!("TokenizationErrors: {:?}", errs);
        return tc_errs;
    };

    let (ast, parse_errors) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        println!("ParsingErrors: {:?}", parse_errors);
        return tc_errs;
    };
    
    xs_tc(path, &ast, &mut None, type_env, local_envs, groups, &mut tc_errs);
    
    tc_errs
}