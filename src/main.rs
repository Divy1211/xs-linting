use std::{env};
use std::collections::{HashMap, HashSet};
use crate::lint::gen_info_from_path::gen_info_from_path;

pub mod parsing;
pub mod r#static;
pub mod lint;

fn main() {
    let mut path = env::current_dir().unwrap();
    let filename = env::args().nth(1).expect("Filename not provided");
    path.push(filename);

    let mut type_env= HashMap::new();
    let mut local_envs = HashMap::new();
    let mut groups = HashSet::new();
    
    gen_info_from_path(&mut type_env, &mut local_envs, &mut groups, path);

    println!("tenv: {:?}", type_env);
    println!("lenvs: {:?}", local_envs);
    println!("grps: {:?}", groups);
}
