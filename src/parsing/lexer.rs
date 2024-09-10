pub mod token;
pub mod literal;
pub mod operator;
pub mod punctuation;
pub mod keyword;
pub mod comment;

use chumsky::prelude::*;
use token::Token;
use crate::parsing::lexer::comment::comment;
use crate::parsing::lexer::keyword::keyword;
use crate::parsing::lexer::literal::literal;
use crate::parsing::lexer::operator::operator;
use crate::parsing::lexer::punctuation::punctuation;
use crate::parsing::span::{Span, Spanned};

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
        // .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
