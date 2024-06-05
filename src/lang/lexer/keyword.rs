use chumsky::prelude::*;
use crate::lang::span::{Span, Spanned};
use crate::lang::lexer::tokens::Token;

pub fn keyword() -> impl Parser<char, Spanned<Token>, Error = Simple<char>> {
    let keywords: [(&str, fn(Span) -> Spanned<Token>); 38] = [
        ("vector", |span| Spanned::new(Token::Vector, span)),
        ("include", |span| Spanned::new(Token::Include, span)),
        ("switch", |span| Spanned::new(Token::Switch, span)),
        ("case", |span| Spanned::new(Token::Case, span)),
        ("while", |span| Spanned::new(Token::While, span)),
        ("break", |span| Spanned::new(Token::Break, span)),
        ("default", |span| Spanned::new(Token::Default, span)),
        ("rule", |span| Spanned::new(Token::Rule, span)),
        ("if", |span| Spanned::new(Token::If, span)),
        ("then", |span| Spanned::new(Token::Then, span)),
        ("else", |span| Spanned::new(Token::Else, span)),
        ("goto", |span| Spanned::new(Token::Goto, span)),
        ("label", |span| Spanned::new(Token::Label, span)),
        ("for", |span| Spanned::new(Token::For, span)),
        ("dbg", |span| Spanned::new(Token::Dbg, span)),
        ("return", |span| Spanned::new(Token::Return, span)),
        ("void", |span| Spanned::new(Token::Void, span)),
        ("int", |span| Spanned::new(Token::Int, span)),
        ("float", |span| Spanned::new(Token::Float, span)),
        ("string", |span| Spanned::new(Token::String, span)),
        ("const", |span| Spanned::new(Token::Const, span)),
        ("priority", |span| Spanned::new(Token::Priority, span)),
        ("minInterval", |span| Spanned::new(Token::MinInterval, span)),
        ("maxInterval", |span| Spanned::new(Token::MaxInterval, span)),
        ("highFrequency", |span| Spanned::new(Token::HighFrequency, span)),
        ("active", |span| Spanned::new(Token::Active, span)),
        ("inactive", |span| Spanned::new(Token::Inactive, span)),
        ("group", |span| Spanned::new(Token::Group, span)),
        ("infiniteLoopLimit", |span| Spanned::new(Token::InfiniteLoopLimit, span)),
        ("infiniteRecursionLimit", |span| Spanned::new(Token::InfiniteRecursionLimit, span)),
        ("breakpoint", |span| Spanned::new(Token::Breakpoint, span)),
        ("static", |span| Spanned::new(Token::Static, span)),
        ("continue", |span| Spanned::new(Token::Continue, span)),
        ("extern", |span| Spanned::new(Token::Extern, span)),
        ("export", |span| Spanned::new(Token::Export, span)),
        ("runImmediately", |span| Spanned::new(Token::RunImmediately, span)),
        ("mutable", |span| Spanned::new(Token::Mutable, span)),
        ("class", |span| Spanned::new(Token::Class, span)),
    ];
    
    choice(keywords.map(|(kw, token_fn)| {
        text::keyword(kw)
            .map_with_span(move |_, span| token_fn(span))
            .padded()
    }))
}
