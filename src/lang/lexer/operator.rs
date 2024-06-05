use chumsky::prelude::*;
use crate::lang::lexer::tokens::Token;
use crate::lang::span::Spanned;

pub fn operator() -> impl Parser<char, Spanned<Token>, Error = Simple<char>> {
    let dplus = just("++")
        .map_with_span(|_, span| Spanned::new(Token::DPlus, span)).padded();
    let plus = just("+")
        .map_with_span(|_, span| Spanned::new(Token::Plus, span)).padded();
    let dminus = just("--")
        .map_with_span(|_, span| Spanned::new(Token::DMinus, span)).padded();
    let minus = just("-")
        .map_with_span(|_, span| Spanned::new(Token::Minus, span)).padded();
    let star = just("*")
        .map_with_span(|_, span| Spanned::new(Token::Star, span)).padded();
    let fslash = just("/")
        .map_with_span(|_, span| Spanned::new(Token::FSlash, span)).padded();
    let pcent = just("%")
        .map_with_span(|_, span| Spanned::new(Token::FSlash, span)).padded();
    let le = just("<=")
        .map_with_span(|_, span| Spanned::new(Token::LE, span)).padded();
    let lt = just("<")
        .map_with_span(|_, span| Spanned::new(Token::LT, span)).padded();
    let ge = just(">=")
        .map_with_span(|_, span| Spanned::new(Token::GE, span)).padded();
    let gt = just(">")
        .map_with_span(|_, span| Spanned::new(Token::GT, span)).padded();
    let deq = just("==")
        .map_with_span(|_, span| Spanned::new(Token::Deq, span)).padded();
    let neq = just("!=")
        .map_with_span(|_, span| Spanned::new(Token::Deq, span)).padded();
    let and = just("&&")
        .map_with_span(|_, span| Spanned::new(Token::Deq, span)).padded();
    let or = just("||")
        .map_with_span(|_, span| Spanned::new(Token::Deq, span)).padded();
    
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