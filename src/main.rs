use std::collections::{HashMap, HashSet};
use std::env;
use std::path::PathBuf;
use crate::lint::gen_info::{gen_info_from_path, gen_info_from_src};

pub mod parsing;
pub mod r#static;
pub mod lint;

fn main() {
    let mut type_env= HashMap::new();
    let mut local_envs = HashMap::new();
    let mut groups = HashSet::new();

    let mut path = PathBuf::from(r"prelude.xs");
    let prelude = include_str!(r"./prelude.xs");
    gen_info_from_src(&mut type_env, &mut local_envs, &mut groups, &path, prelude);
    
    let filename = env::args().nth(1).expect("Filename not provided");
    path.push(filename);
    gen_info_from_path(&mut type_env, &mut local_envs, &mut groups, path);
    // println!("tenv: {:?}", type_env);
    // println!("lenvs: {:?}", local_envs);
    // println!("grps: {:?}", groups);
}
