use chumsky::prelude::*;
use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::parser::statement::statement;
use crate::lang::span::{Span, Spanned};

pub mod expression;
pub mod parser_input;
pub mod statement;

pub fn parser<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Vec<Spanned<ASTreeNode>>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    statement()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
