use std::fmt;
use crate::parsing::lexer::token::Token;
use fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Vec,
    Void,

    // not real types in XS
    Label,
    Rule,
    Group,
    Func { is_mutable: bool, type_sign: Vec<Type> },
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

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Str => write!(f, "string"),
            Type::Vec => write!(f, "vector"),
            Type::Void => write!(f, "void"),

            Type::Label => write!(f, "label"),
            Type::Rule => write!(f, "rule"),
            Type::Group => write!(f, "group"),
            Type::Func { is_mutable, type_sign} => write!(
                f,
                "{}{}",
                if *is_mutable { "mut " } else { "" },
                type_sign
                    .iter()
                    .map(|type_| type_.to_string())
                    .collect::<Vec<String>>()
                    .join(" -> ")
            )
        }
    }
}