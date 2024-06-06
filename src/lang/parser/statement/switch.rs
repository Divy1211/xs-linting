use chumsky::prelude::*;
use crate::lang::ast::astree::{ASTreeNode, Body};
use crate::lang::ast::expr::Expr;
use crate::lang::lexer::token::Token;
use crate::lang::parser::expression::expression;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::parser::statement::body::body;
use crate::lang::span::{Span, Spanned};

pub fn switch<'tokens>(
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
    let case = just(Token::Case)
        .ignore_then(expression())
        .then_ignore(just(Token::Colon))
        .then(body(statement.clone()));
    
    let default = just([Token::Default, Token::Colon])
        .ignore_then(body(statement.clone()));
    
    just(Token::Switch)
        .ignore_then(expression().delimited_by(just(Token::LParen), just(Token::RParen)))
        .then(
            case.clone().repeated().collect::<Vec<(Spanned<Expr>, Body)>>()
                .then(default.then(case.repeated().collect::<Vec<(Spanned<Expr>, Body)>>()).or_not())
                .delimited_by(just(Token::LBrace), just(Token::RBrace))
        ).map_with(|(clause, (mut cases, cases2)), info| {

        let default = match cases2 {
            Some((default, cases2)) => {
                cases.extend(cases2);
                Some(default)
            },
            None => None,
        };

        (ASTreeNode::Switch { clause, cases, default }, info.span())
    })
}
