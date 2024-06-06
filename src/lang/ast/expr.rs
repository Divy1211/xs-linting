use crate::lang::ast::identifier::Identifier;
use crate::lang::ast::literal::Literal;

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Identifier(Identifier),
    Parenthesis(Box<Expr>),
    FnCall {
        name: Identifier,
        args: Vec<Expr>
    },
    
    Star(Box<Expr>, Box<Expr>),
    FSlash(Box<Expr>, Box<Expr>),
    PCent(Box<Expr>, Box<Expr>),
    
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),

    LT(Box<Expr>, Box<Expr>),
    GT(Box<Expr>, Box<Expr>),
    LE(Box<Expr>, Box<Expr>),
    GE(Box<Expr>, Box<Expr>),

    EQ(Box<Expr>, Box<Expr>),
    NE(Box<Expr>, Box<Expr>),

    And(Box<Expr>, Box<Expr>),
    
    Or(Box<Expr>, Box<Expr>),
}
