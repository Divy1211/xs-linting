use chumsky::error::Rich;
use chumsky::{extra, Parser};
use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::span::{Span, Spanned};

pub mod expression;
pub mod parser_input;
pub mod statement;

// pub fn parser<'tokens>(
// ) -> impl Parser<
//     'tokens,
//     ParserInput<'tokens>,
//     Vec<Spanned<ASTreeNode>>,
//     extra::Err<Rich<'tokens, Token, Span>>,
// > + Clone {
//     
// }
