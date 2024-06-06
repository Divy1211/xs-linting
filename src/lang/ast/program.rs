use crate::lang::ast::identifier::Identifier;
use crate::lang::ast::literal::Literal;
use crate::lang::ast::statement::Body;
use crate::lang::ast::type_::Type;

#[derive(Debug)]
pub struct Arg {
    type_: Type,
    name: Identifier,
    value: Literal
}

#[derive(Debug)]
pub enum ASTreeNode {
    Include { path: String },
    VarDef {
        is_extern: bool,
        is_const: bool,
        is_static: bool,
        type_: Type,
        name: Identifier,
        value: Option<Literal>,
    },
    RuleDef {
        name: Identifier,
        is_active: bool,
        group_name: Literal, // always Str
    },
    FnDef {
        is_extern: bool,
        is_mutable: bool,
        return_type: Type,
        name: Identifier,
        arguments: Vec<Arg>,
        body: Body,
    },
}
