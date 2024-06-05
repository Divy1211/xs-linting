use chumsky::prelude::*;
use crate::lang::ast::literal::{Identifier, Literal};
use crate::lang::lexer::tokens::Token;
use crate::lang::span::Spanned;

pub fn literal() -> impl Parser<char, Spanned<Token>, Error = Simple<char>> {
    let int = text::int(10)
        .map_with_span(|num: String, span| {
            Spanned::new(Token::Literal(Literal::Int(num.parse().unwrap())), span)
        }).padded();

    let float = text::int(10)
        .then_ignore(just('.'))
        .then(text::digits(10))
        .map_with_span(|(whole, fraction), span| {
            Spanned::new(
                Token::Literal(Literal::Float(format!("{}.{}", whole, fraction).parse().unwrap())),
                span
            )
        }).padded();

    let bool = just("true").or(just("false"))
        .map_with_span(|val, span| {
            Spanned::new(Token::Literal(Literal::Bool(val.parse().unwrap())), span)
        }).padded();

    let string = just('"')
        .ignore_then(
            choice((
                none_of("\\\""),
                just("\\").ignore_then(any()).map(|c| match c {
                    'n' => '\n',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    _ => c,
                })
            )).repeated()
        )
        .then_ignore(just('"'))
        .map_with_span(|chars, span| {
            let string: String = chars.into_iter().collect();
            Spanned::new(Token::Literal(Literal::Str(string)), span)
        }).padded();

    let id = text::ident()
        .map_with_span(|name, span| {
            Spanned::new(Token::Identifier(Identifier(name)), span)
        }).padded();
    
    choice((
        float,
        int,
        bool,
        string,
        id,
    ))
}