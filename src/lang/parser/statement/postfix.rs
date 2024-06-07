use chumsky::prelude::*;
use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::span::{Span, Spanned};

pub fn postfix<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    select! { Token::Identifier(id) => id }
        .map_with(|id, info| (id, info.span()))
        .then(one_of([Token::DMinus, Token::DPlus]))
        .then_ignore(just(Token::SColon))
        .map_with(|(name, tok), info| (match tok {
            Token::DMinus => ASTreeNode::PostDMinus(name),
            _             => ASTreeNode::PostDPlus(name),
        }, info.span()))
}