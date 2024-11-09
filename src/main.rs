use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use crate::cli::parse_args;
use crate::lint::gen_info::{gen_info_from_path, gen_info_from_src};

pub mod parsing;
pub mod r#static;
pub mod lint;
pub mod cli;

fn main() {
    let (filepath, ignores) = match parse_args() {
        Some(filepath) => { filepath }
        None => { return; },
    };

    let mut type_env= HashMap::new();
    let mut local_envs = HashMap::new();
    let mut groups = HashSet::new();

    let path = PathBuf::from(r"prelude.xs");
    let prelude = include_str!(r"./prelude.xs");

    gen_info_from_src(&mut type_env, &mut local_envs, &mut groups, &path, prelude, &ignores);

    gen_info_from_path(&mut type_env, &mut local_envs, &mut groups, filepath, &ignores);
}
