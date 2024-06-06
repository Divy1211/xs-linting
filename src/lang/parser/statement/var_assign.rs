use chumsky::prelude::*;
use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::expression::expression;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::span::{Span, Spanned};

pub fn var_assign<'tokens>(
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    select! { Token::Identifier(id) => id }
        .map_with(|id, info| (id, info.span()))
        .then_ignore(just(Token::Eq))
        .then(expression())
        .then_ignore(just(Token::SColon))
        .map_with(|(name,  value), info| {
            (ASTreeNode::VarAssign {
                name,
                value,
            }, info.span())
        })
}