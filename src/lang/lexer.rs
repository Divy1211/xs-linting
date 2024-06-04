use chumsky::prelude::*;
use tokens::Token;
use crate::lang::lexer::keyword::keyword;
use crate::lang::lexer::literal::literal;
use crate::lang::lexer::operator::operator;
use crate::lang::lexer::punctuation::punctuation;

pub mod tokens;
pub mod literal;
pub mod operator;
pub mod punctuation;
pub mod keyword;

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let comment = just("//")
        .ignore_then(none_of("\r\n").repeated().padded())
        .or(just("/*").ignore_then(none_of("*/").repeated().padded()))
        .map(|chars: Vec<char>| {
            let string: String = chars.into_iter().collect();
            Token::Comment(string)
        });
    
    choice((
        comment,
        keyword(),
        literal(),
        operator(),
        punctuation(),
    ))
        .repeated()
        .then_ignore(end())
}
