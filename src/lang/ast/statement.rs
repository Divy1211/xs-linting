use std::collections::HashMap;
use crate::lang::ast::expr::Expr;
use crate::lang::ast::literal::{Identifier, Literal};
use crate::lang::ast::type_::Type;

#[derive(Debug)]
pub enum Body {
    Statements(Vec<Statement>),
}

#[derive(Debug)]
pub enum Statement {
    VarDef {
        is_const: bool,
        is_static: bool,
        type_: Type,
        name: Identifier,
        value: Option<Expr>,
    },
    VarAssign {
        name: Identifier,
        value: Expr,
    },
    IfElse {
        condition: Expr,
        then: BodyStatement,
        else_: Option<BodyStatement>,
    },
    While {
        condition: Expr,
        body: BodyStatement,
    },
    For {
        var: Box<Statement>, // always VarAssign
        condition: Expr,
        body: BodyStatement,
    },
    Switch {
        clause: Expr,
        cases: HashMap<Literal, BodyStatement>,
    },
    PostDPlus(Identifier),
    PostDMinus(Identifier),
    Discarded(Expr),
    Break,
    Continue,
    Return (Expr),
    DocStr(String),
}

#[derive(Debug)]
pub enum BodyStatement {
    Body(Body),
    Statement(Box<Statement>),
}
