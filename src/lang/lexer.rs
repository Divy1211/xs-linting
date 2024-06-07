pub mod token;
pub mod literal;
pub mod operator;
pub mod punctuation;
pub mod keyword;
mod comment;

use chumsky::prelude::*;
use token::Token;
use crate::lang::lexer::comment::comment;
use crate::lang::lexer::keyword::keyword;
use crate::lang::lexer::literal::literal;
use crate::lang::lexer::operator::operator;
use crate::lang::lexer::punctuation::punctuation;
use crate::lang::span::{Span, Spanned};

pub fn lexer<'src>() -> impl Parser<
    'src, &'src str, Vec<Spanned<Token>>, extra::Err<Rich<'src, char, Span>>
> {
    choice((
        comment(),
        literal(),
        keyword(),
        operator(),
        punctuation(),
    ))
        .map_with(|tok, info| (tok, info.span()))
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
