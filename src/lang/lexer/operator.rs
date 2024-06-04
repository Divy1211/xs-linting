use chumsky::prelude::*;
use crate::lang::lexer::tokens::Token;

pub fn operator() -> impl Parser<char, Token, Error = Simple<char>> {
    let dplus = just("++").to(Token::DPlus).padded();
    let plus = just("+").to(Token::Plus).padded();
    let dminus = just("--").to(Token::DMinus).padded();
    let minus = just("-").to(Token::Minus).padded();
    let star = just("*").to(Token::Star).padded();
    let fslash = just("/").to(Token::FSlash).padded();
    let pcent = just("%").to(Token::FSlash).padded();
    let le = just("<=").to(Token::LE).padded();
    let lt = just("<").to(Token::LT).padded();
    let ge = just(">=").to(Token::GE).padded();
    let gt = just(">").to(Token::GT).padded();
    let deq = just("==").to(Token::Deq).padded();
    let neq = just("!=").to(Token::Deq).padded();
    let and = just("&&").to(Token::Deq).padded();
    let or = just("||").to(Token::Deq).padded();
    
    choice((
        dplus,
        plus,
        dminus,
        minus,
        star,
        fslash,
        pcent,
        le,
        lt,
        ge,
        gt,
        deq,
        neq,
        and,
        or,
    ))
}