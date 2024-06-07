use crate::lang::ast::expr::Expr;
use crate::lang::ast::identifier::Identifier;
use crate::lang::ast::type_::Type;
use crate::lang::span::Spanned;

#[derive(Debug, Clone)]
pub struct Param {
    pub type_: Type,
    pub name: Spanned<Identifier>,
    pub default: Spanned<Expr>
}
