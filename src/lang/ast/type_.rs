use crate::lang::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Vec,
    Void,
    Label,
}

impl Type {
    pub fn from_tok(tok: Token) -> Self {
        match tok {
            Token::Int    => Type::Int,
            Token::Bool   => Type::Bool,
            Token::Float  => Type::Float,
            Token::String => Type::Str,
            Token::Vector => Type::Vec,
            Token::Void   => Type::Void,
            _             => panic!("Non type token cannot be converted to type!"),
        }
    }
}