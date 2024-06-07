use chumsky::error::Rich;
use chumsky::prelude::*;
use crate::lang::ast::comment::Comment;
use crate::lang::lexer::token::Token;
use crate::lang::span::{Span};

pub fn comment<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    just("//").ignore_then(none_of("\r\n").repeated())
        .or(just("/*").ignore_then(none_of("*/").repeated()))
        .to_slice()
        .map(|val: &str| Token::Comment(Comment(val.to_string())))
}