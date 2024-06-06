use crate::lang::ast::identifier::Identifier;
use crate::lang::ast::literal::Literal;
use crate::lang::ast::type_::Type;

#[derive(Debug, Clone)]
pub struct Param {
    pub type_: Type,
    pub name: Identifier,
    pub value: Literal
}

impl Param {
    fn new(type_: Type, name: Identifier, value: Literal) -> Self {
        Param { type_, name, value }
    }
}
