use std::collections::HashMap;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::type_::Type;

pub mod statement;
pub mod expression;
pub mod util;

pub type TypeEnv = HashMap<Identifier, Type>;
