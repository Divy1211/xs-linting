use std::collections::{HashMap, HashSet};
use chumsky::container::Container;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::type_::Type;
use crate::parsing::span::Span;

pub mod statement;
pub mod expression;
pub mod util;
pub mod statements;

pub type TypeEnv = HashMap<Identifier, (Type, Span)>; // todo: add filepath
pub type LocalEnv = HashMap<Identifier, Vec<(TypeEnv, Span)>>;
pub type Groups = HashSet<String>;

pub fn env_get<'src>(
    local_env: &'src Option<TypeEnv>,
    type_env: &'src TypeEnv,
    id: &Identifier
) -> Option<&'src (Type, Span)> {
    match local_env {
        Some(env) => { env.get(id).or_else(|| type_env.get(id)) }
        None => { type_env.get(id) }
    }
}

pub fn env_set<'src>(
    local_env: &'src mut Option<TypeEnv>,
    type_env: &'src mut TypeEnv,
    id: &Identifier,
    val: (Type, Span),
) {
    match local_env {
        Some(env) => { env.push((id.clone(), val)) }
        None => { type_env.push((id.clone(), val)) }
    }
}