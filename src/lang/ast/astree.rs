use std::collections::HashMap;
use crate::lang::ast::expr::Expr;
use crate::lang::ast::identifier::Identifier;
use crate::lang::ast::literal::Literal;
use crate::lang::ast::param::Param;
use crate::lang::ast::type_::Type;
use crate::lang::span::Spanned;

#[derive(Debug, Clone)]
pub enum Body {
    Statements(Vec<Spanned<ASTreeNode>>),
}

#[derive(Debug, Clone)]
pub enum BodyStatement {
    Body(Body),
    Statement(Box<Spanned<ASTreeNode>>),
}

#[derive(Debug, Clone)]
pub enum RuleFreq {
    HighFreq,
    Interval(i64, i64)
}

#[derive(Debug, Clone)]
pub enum ASTreeNode {
    Include(String),
    VarDef {
        is_extern: bool, // no extern in locals
        is_const: bool, // only literals can be assigned to consts, no exprs allowed
        is_static: bool,
        type_: Type,
        name: Identifier,
        value: Option<Spanned<Expr>>, // only literals allowed in top level, strings are bugged, vecs are fine
    },
    RuleDef {
        name: Identifier,
        is_active: bool,
        run_immediately: bool,
        frequency: Spanned<RuleFreq>,
        priority: Spanned<i64>,
        group_name: Spanned<Literal>, // always Str
        body: Body,
    },
    FnDef {
        is_extern: bool,
        is_mutable: bool,
        return_type: Type,
        name: Identifier,
        parameters: Vec<Param>,
        body: Body,
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
        var: Box<ASTreeNode>, // always VarAssign
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
