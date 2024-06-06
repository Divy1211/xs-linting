use crate::lang::ast::expr::Expr;
use crate::lang::ast::identifier::Identifier;
use crate::lang::ast::literal::Literal;
use crate::lang::ast::param::Param;
use crate::lang::ast::type_::Type;
use crate::lang::span::Spanned;

#[derive(Debug, Clone)]
pub enum Body {
    Block(Vec<Spanned<ASTreeNode>>),
    Single(Box<Spanned<ASTreeNode>>),
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
        name: Spanned<Identifier>,
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
        name: Spanned<Identifier>,
        value: Spanned<Expr>,
    },
    IfElse {
        condition: Spanned<Expr>,
        consequent: Body,
        alternate: Option<Body>,
    },
    While {
        condition: Spanned<Expr>,
        body: Body,
    },
    For {
        var: Box<Spanned<ASTreeNode>>, // always VarAssign
        condition: Spanned<Expr>,
        body: Body,
    },
    Switch {
        clause: Spanned<Expr>,
        cases: Vec<(Spanned<Expr>, Body)>, // expr can only be literal. todo: are vecs allowed?
        default: Option<Body>,
    },
    PostDPlus(Identifier),
    PostDMinus(Identifier),
    Discarded(Expr),
    Break,
    Continue,
    Return (Expr),
    DocStr(String),
    
    Error(String),
}
