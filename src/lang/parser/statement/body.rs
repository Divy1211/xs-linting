use chumsky::prelude::*;
use crate::lang::ast::astree::{ASTreeNode, Body};
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::span::{Span, Spanned};


pub fn body<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<ASTreeNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<Body>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    let block = statement.clone()
        .repeated()
        .collect::<Vec<Spanned<ASTreeNode>>>()
        .delimited_by(just(Token::LBrace), just(Token::RBrace))
        .map_with(|stmts, info| (
            Body::Block(stmts), info.span()
        ));
    
    let single = statement
        .map_with(|stmt, info| (
            Body::Single(Box::new(stmt)), info.span()
        ));
    
    choice((
        block,
        single,
    ))
}