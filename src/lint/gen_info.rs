use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use chumsky::input::Input;
use chumsky::Parser;

use crate::lint::gen_errs::{gen_errs, gen_xs_errs};
use crate::parsing::lexer::lexer;
use crate::parsing::parser::parser;
use crate::r#static::type_check::{LocalEnv, TypeEnv};
use crate::r#static::type_check::statements::xs_tc;
use crate::r#static::xs_error::XSError;

pub fn gen_info_from_path(
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut HashSet<String>,
    path: PathBuf
) {
    let src = fs::read_to_string(&path).expect("Failed to read file");
    let filename = path.to_str().unwrap();
    
    let errs = gen_info_from_src(
        type_env, local_envs, groups,
        &path, &src
    );

    gen_xs_errs(&errs, filename, &src)
}

pub fn gen_info_from_src(
    type_env: &mut TypeEnv,
    local_envs: &mut LocalEnv,
    groups: &mut HashSet<String>,
    path: &PathBuf,
    src: &str
) -> Vec<XSError> {
    let (tokens, errs) = lexer()
        .parse(src)
        .into_output_errors();

    let mut tc_errs = vec![];
    
    let Some(tokens) = tokens else {
        gen_errs("TokenizationError", &errs, path, src);
        return tc_errs;
    };

    let (ast, errs) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        gen_errs("ParsingError", &errs, path, src);
        return tc_errs;
    };
    
    xs_tc(path, &ast, &mut None, type_env, local_envs, groups, &mut tc_errs);
    
    tc_errs
}