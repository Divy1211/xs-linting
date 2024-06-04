use chumsky::prelude::*;
use crate::lang::ast::literal::{Identifier, Literal};
use crate::lang::lexer::tokens::Token;

pub fn literal() -> impl Parser<char, Token, Error = Simple<char>> {
    let int = text::int(10)
        .map(|num: String| Token::Literal(Literal::Int(num.parse().unwrap())))
        .padded();

    let float = text::int(10)
        .then_ignore(just('.'))
        .then(text::digits(10))
        .map(|(whole, fraction): (String, String)| {
            Token::Literal(Literal::Float(format!("{}.{}", whole, fraction).parse().unwrap()))
        })
        .padded();

    let bool = just("true").or(just("false"))
        .map(|val: &str| Token::Literal(Literal::Bool(val.parse().unwrap())))
        .padded();

    let string = just('"')
        .ignore_then(
            choice((
                none_of("\\\""),
                just("\\").ignore_then(any()).map(|c| match c {
                    'n' => '\n',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    _ => c,
                })
            )).repeated()
        )
        .then_ignore(just('"'))
        .map(|chars: Vec<char>| {
            let string: String = chars.into_iter().collect();
            Token::Literal(Literal::Str(string))
        })
        .padded();

    let num = just("-").or_not().then(float.or(int))
        .map(|(sign, num)| match num {
            Token::Literal(n) => { match n {
                Literal::Int(i) => { match sign { Some(_) => -i as f64, None => i as f64 } }
                Literal::Float(f) => { match sign { Some(_) => -f, None => f } }
                _ => { 0f64 }
            }}
            _ => { 0f64 }
        });

    let vec = just("vec(")
        .ignore_then(num)
        .then_ignore(just(",").padded())
        .then(num)
        .then_ignore(just(",").padded())
        .then(num)
        .then_ignore(just(")").padded())
        .map(|((x, y), z)| Token::Literal(Literal::Vec { x, y, z }))
        .padded();

    let id = text::ident().map(|name: String| Token::Identifier(Identifier(name))).padded();
    
    choice((float, int, bool, string, vec, id))
}