use chumsky::prelude::*;

use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::expression::expression;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::span::{Span, Spanned};

pub fn return_<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just(Token::Return)
        .ignore_then(expression().or_not())
        .then_ignore(just(Token::SColon))
        .map_with(|expr, info| (
            ASTreeNode::Return (expr), info.span()
        ))
}
