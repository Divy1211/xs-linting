use chumsky::prelude::*;
use crate::lang::ast::astree::ASTreeNode;
use crate::lang::ast::literal::Literal;
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::span::{Span, Spanned};

pub fn include<'tokens>(
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just([Token::Include])
        .ignore_then(
            select! { Token::Literal(Literal::Str(path)) => ASTreeNode::Include(path) }
                .map_with(|node, info| (node, info.span()))
        )
}