use chumsky::error::Rich;
use chumsky::prelude::*;
use crate::parsing::ast::comment::Comment;
use crate::parsing::lexer::token::Token;
use crate::parsing::span::{Span};

pub fn comment<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    just("//").ignore_then(none_of("\r\n").repeated())
        .or(just("/*").ignore_then(none_of("*/").repeated()))
        .to_slice()
        .map(|val: &str| Token::Comment(Comment::new(val)))
}