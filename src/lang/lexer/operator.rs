use chumsky::prelude::*;
use crate::lang::lexer::token::Token;
use crate::lang::span::{Span};

pub fn operator<'src>(
) -> impl Parser<'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>> {
    one_of("+-*/%<>=!&|")
        .repeated().at_least(1)
        .to_slice()
        .try_map(|val, span| match val {
            "++" => Ok(Token::DPlus),
            "+"  => Ok(Token::Plus),
            "--" => Ok(Token::DMinus),
            "-"  => Ok(Token::Minus),
            "*"  => Ok(Token::Star),
            "/"  => Ok(Token::FSlash),
            "%"  => Ok(Token::FSlash),
            "<=" => Ok(Token::LE),
            "<"  => Ok(Token::LT),
            ">=" => Ok(Token::GE),
            ">"  => Ok(Token::GT),
            "==" => Ok(Token::Deq),
            "!=" => Ok(Token::Neq),
            "&&" => Ok(Token::And),
            "||" => Ok(Token::Or),
            _    => Err(Rich::custom(span, "Invalid operator")),
        })
}