use chumsky::prelude::*;
use crate::lang::lexer::tokens::Token;

pub fn punctuation() -> impl Parser<char, Token, Error = Simple<char>> {
    let eq = just("=").to(Token::Eq).padded();
    let lparen = just("(").to(Token::LParen).padded();
    let rparen = just(")").to(Token::RParen).padded();
    let lbrace = just("{").to(Token::LBrace).padded();
    let rbrace = just("}").to(Token::RBrace).padded();
    let scolon = just(";").to(Token::SColon).padded();
    let colon = just(":").to(Token::Colon).padded();
    let comma = just(",").to(Token::Comma).padded();
    let dot = just(".").to(Token::Dot).padded();
    
    choice((
        eq,
        lparen,
        rparen,
        lbrace,
        rbrace,
        scolon,
        colon,
        comma,
        dot,
    ))
}
