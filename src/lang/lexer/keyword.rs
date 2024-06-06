use chumsky::prelude::*;
use crate::lang::ast::identifier::Identifier;
use crate::lang::span::{Span};
use crate::lang::lexer::token::Token;

pub fn keyword<'src>(
) -> impl Parser<'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>> {
    text::ascii::ident().map(|ident| match ident {
        "vector"                 => Token::Vector,
        "include"                => Token::Include,
        "switch"                 => Token::Switch,
        "case"                   => Token::Case,
        "while"                  => Token::While,
        "break"                  => Token::Break,
        "default"                => Token::Default,
        "rule"                   => Token::Rule,
        "if"                     => Token::If,
        "then"                   => Token::Then,
        "else"                   => Token::Else,
        "goto"                   => Token::Goto,
        "label"                  => Token::Label,
        "for"                    => Token::For,
        "dbg"                    => Token::Dbg,
        "return"                 => Token::Return,
        "void"                   => Token::Void,
        "int"                    => Token::Int,
        "float"                  => Token::Float,
        "string"                 => Token::String,
        "const"                  => Token::Const,
        "priority"               => Token::Priority,
        "minInterval"            => Token::MinInterval,
        "maxInterval"            => Token::MaxInterval,
        "highFrequency"          => Token::HighFrequency,
        "active"                 => Token::Active,
        "inactive"               => Token::Inactive,
        "group"                  => Token::Group,
        "infiniteLoopLimit"      => Token::InfiniteLoopLimit,
        "infiniteRecursionLimit" => Token::InfiniteRecursionLimit,
        "breakpoint"             => Token::Breakpoint,
        "static"                 => Token::Static,
        "continue"               => Token::Continue,
        "extern"                 => Token::Extern,
        "export"                 => Token::Export,
        "runImmediately"         => Token::RunImmediately,
        "mutable"                => Token::Mutable,
        "class"                  => Token::Class,
        _                        => Token::Identifier(Identifier(ident.to_string()))
    })
}
