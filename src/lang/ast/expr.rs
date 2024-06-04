use crate::lang::ast::literal::{Identifier, Literal};

#[derive(Debug)]
pub enum E7 {
    Literal(Literal),
    Identifier(Identifier),
    Parenthesis(Box<Expr>),
    FnCall {
        name: Identifier,
        args: Vec<Box<Expr>>
    }
}

#[derive(Debug)]
pub enum E6 {
    E7(Box<E7>),
    Star(Box<E6>, Box<E7>),
    FSlash(Box<E6>, Box<E7>),
    PCent(Box<E6>, Box<E7>),
}

#[derive(Debug)]
pub enum E5 {
    E6(Box<E6>),
    Plus(Box<E5>, Box<E6>),
    Minus(Box<E5>, Box<E6>),
}

#[derive(Debug)]
pub enum E4 {
    E5(Box<E5>),
    LT(Box<E4>, Box<E5>),
    GT(Box<E4>, Box<E5>),
    LE(Box<E4>, Box<E5>),
    GE(Box<E4>, Box<E5>),
}

#[derive(Debug)]
pub enum E3 {
    E4(Box<E4>),
    EQ(Box<E3>, Box<E4>),
    NE(Box<E3>, Box<E4>),
}

#[derive(Debug)]
pub enum E2 {
    E3(Box<E3>),
    And(Box<E2>, Box<E3>),
}

#[derive(Debug)]
pub enum E1 {
    E2(Box<E2>),
    Or(Box<E1>, Box<E2>),
}

#[derive(Debug)]
pub enum Expr {
    E1(Box<E1>),
}
