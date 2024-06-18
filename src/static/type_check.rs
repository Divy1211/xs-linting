use std::collections::{HashMap, HashSet};
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::type_::Type;

pub mod statement;
pub mod expression;
pub mod util;
pub mod statements;

pub type TypeEnv = HashMap<Identifier, Type>;
pub type Groups = HashSet<String>;
