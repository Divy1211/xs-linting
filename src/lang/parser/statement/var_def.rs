use chumsky::prelude::*;
use crate::lang::ast::astree::ASTreeNode;
use crate::lang::ast::type_::Type;
use crate::lang::lexer::token::Token;
use crate::lang::parser::expression::expression;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::span::{Span, Spanned};

pub fn var_def<'tokens>(
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    one_of([Token::Extern, Token::Const, Token::Static]).repeated().collect::<Vec<Token>>()
        .then(one_of([Token::Int, Token::Bool, Token::Float, Token::String, Token::Vector]))
        .then(
            select! { Token::Identifier(id) => id }
            .map_with(|id, info| (id, info.span()))
        )
        .then(just(Token::Eq).ignore_then(expression()).or_not())
        .then_ignore(just(Token::SColon))
        .map_with(|((((mods, type_), name)), value), info| {
            (ASTreeNode::VarDef {
                is_extern: mods.contains(&Token::Extern),
                is_const: mods.contains(&Token::Const),
                is_static: mods.contains(&Token::Static),
                type_: match type_ {
                    Token::Int => Type::Int,
                    Token::Bool => Type::Bool,
                    Token::Float => Type::Float,
                    Token::String => Type::Str,
                    Token::Vector => Type::Vec,
                    _             => Type::Error("TypeUnreachable".to_string()),
                },
                name,
                value,
            }, info.span())
        })
}