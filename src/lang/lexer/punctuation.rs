use chumsky::prelude::*;
use crate::lang::lexer::tokens::Token;
use crate::lang::span::Spanned;

pub fn punctuation() -> impl Parser<char, Spanned<Token>, Error = Simple<char>> {
    let eq = just("=")
        .map_with_span(|_, span| Spanned::new(Token::Eq, span)).padded();
    let lparen = just("(")
        .map_with_span(|_, span| Spanned::new(Token::LParen, span)).padded();
    let rparen = just(")")
        .map_with_span(|_, span| Spanned::new(Token::RParen, span)).padded();
    let lbrace = just("{")
        .map_with_span(|_, span| Spanned::new(Token::LBrace, span)).padded();
    let rbrace = just("}")
        .map_with_span(|_, span| Spanned::new(Token::RBrace, span)).padded();
    let scolon = just(";")
        .map_with_span(|_, span| Spanned::new(Token::SColon, span)).padded();
    let colon = just(":")
        .map_with_span(|_, span| Spanned::new(Token::Colon, span)).padded();
    let comma = just(",")
        .map_with_span(|_, span| Spanned::new(Token::Comma, span)).padded();
    let dot = just(".")
        .map_with_span(|_, span| Spanned::new(Token::Dot, span)).padded();
    
    choice((
        eq,
        lparen,
        rparen,
        lbrace,
        rbrace,
        scolon,
        colon,
        comma,
        dot,
    ))
}
