use chumsky::prelude::*;
use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::expression::expression;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::parser::statement::body::body;
use crate::lang::span::{Span, Spanned};

pub fn if_else<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<ASTreeNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just(Token::If)
        .ignore_then(expression().delimited_by(just(Token::LParen), just(Token::RParen)))
        .then(body(statement.clone()))
        .then(just(Token::Else).ignore_then(body(statement)).or_not())
        .map_with(|
            ((condition, consequent), alternate),
             info
        | {
            (ASTreeNode::IfElse { condition, consequent, alternate }, info.span())
        })
}
