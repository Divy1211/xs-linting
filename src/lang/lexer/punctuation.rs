use chumsky::prelude::*;
use crate::lang::lexer::token::Token;
use crate::lang::span::{Span};

pub fn punctuation<'src>(
) -> impl Parser<'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>> {
    one_of("=(){};:,.")
        .to_slice()
        .try_map(|val, span| match val {
            "=" => Ok(Token::Eq),
            "(" => Ok(Token::LParen),
            ")" => Ok(Token::RParen),
            "{" => Ok(Token::LBrace),
            "}" => Ok(Token::RBrace),
            ";" => Ok(Token::SColon),
            ":" => Ok(Token::Colon),
            "," => Ok(Token::Comma),
            "." => Ok(Token::Dot),
            _   => Err(Rich::custom(span, "PunctuationUnreachable")),
        })
}
