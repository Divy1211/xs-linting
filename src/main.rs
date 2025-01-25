use std::path::PathBuf;

use crate::cli::parse_args;
use crate::lint::{gen_errs_from_path, gen_errs_from_src, print_parse_errs, print_xs_errs};
use crate::r#static::info::{Error, TypeEnv};

pub mod parsing;
mod cli;
pub mod lint;
pub mod r#static;

fn main() {
    let (filepath, ignores) = match parse_args() {
        Some(filepath) => { filepath }
        None => { return; },
    };
    
    let mut type_env= TypeEnv::new();
    
    let path = PathBuf::from(r"prelude.xs");
    let prelude = include_str!(r"./prelude.xs");
    
    gen_errs_from_src(&path, prelude, &mut type_env).expect("Prelude can't produce parse errors");

    gen_errs_from_path(&filepath, &mut type_env).expect("test");
    // match gen_errs_from_path(&filepath, &mut type_env) {
    //     Err(Error::FileErr(msg)) => {},
    //     Err(Error::ParseErrs(errs)) => { print_parse_errs(  ) },
    //     Ok(()) => {
    //         print_xs_errs(type_env.errs());
    //     },
    // };
}
