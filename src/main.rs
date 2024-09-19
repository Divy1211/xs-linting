use std::collections::{HashMap, HashSet};
use std::env;
use std::path::PathBuf;
use crate::lint::gen_info::{gen_info_from_path, gen_info_from_src};

pub mod parsing;
pub mod r#static;
pub mod lint;

include!(concat!(env!("OUT_DIR"), "/build_date.rs"));

fn print_help() {
    println!("Usage: xs-check path/to/script.xs");
    println!("lints the provided XS file with AoE2:DE's flavour of XS");
    println!();
    println!("Options:");
    println!("    -h, --help         show this menu");
    println!("    -v, --version      show binary version & info");
}

fn print_ver() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    
    println!("{name} v{version}");
    println!("Author: {authors}");
    println!("Compiled: {BUILD_DATE}");
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(arg) => {
            match arg.as_str() {
                "--help" | "-h" => { return print_help() }
                "--version" | "-v" => { return print_ver() }
                _ => { arg }
            }
        }
        None => { return print_help() }
    };
    
    
    let mut type_env= HashMap::new();
    let mut local_envs = HashMap::new();
    let mut groups = HashSet::new();

    let mut path = PathBuf::from(r"prelude.xs");
    let prelude = include_str!(r"./prelude.xs");
    gen_info_from_src(&mut type_env, &mut local_envs, &mut groups, &path, prelude);
    
    path.push(env::current_dir().unwrap());
    path.push(filename);
    gen_info_from_path(&mut type_env, &mut local_envs, &mut groups, path);
    // println!("tenv: {:?}", type_env);
    // println!("lenvs: {:?}", local_envs);
    // println!("grps: {:?}", groups);
}
