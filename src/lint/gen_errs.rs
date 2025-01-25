use std::path::PathBuf;
use std::fs;

use chumsky::input::Input;
use chumsky::Parser;

use crate::parsing::lexer::lexer;
use crate::parsing::lexer::token::Token;
use crate::parsing::parser::parser;
use crate::r#static::info::type_env::TypeEnv;
use crate::r#static::info::error::{Error, ParseError};
use crate::r#static::type_check::statements::xs_tc;


pub fn gen_errs_from_path(
    path: &PathBuf,
    type_env: &mut TypeEnv,
) -> Result<(), Error> {
    let src = match fs::read_to_string(&path) {
        Ok(src) => {src}
        Err(err) => {
            let path = path.to_str().unwrap();
            return Err(Error::FileErr(format!("Failed to read path {path}, details: {err}")))
        }
    };

    gen_errs_from_src(path, &src, type_env)
        .map_err(Error::ParseErrs)
}

pub fn gen_errs_from_src(
    path: &PathBuf,
    src: &str,
    type_env: &mut TypeEnv,
) -> Result<(), Vec<ParseError>> {
    let (tokens, errs) = lexer()
        .parse(src)
        .into_output_errors();

    let Some(mut tokens) = tokens else {
        return Err(
            errs.iter()
                .map(ParseError::lex_err)
                .collect()
        );
    };

    tokens = tokens.into_iter()
        .filter(|tok| match tok { (Token::Comment(_), _) => { false }, _ => { true } })
        .collect();

    let (ast, errs) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        return Err(
            errs.iter()
                .map(ParseError::parse_err)
                .collect()
        );
    };

    xs_tc(path, &ast, type_env);
    
    Ok(())
}