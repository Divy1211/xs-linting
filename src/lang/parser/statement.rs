use chumsky::prelude::*;

use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::parser::statement::for_::for_;
use crate::lang::parser::statement::include::include;
use crate::lang::parser::statement::if_else::if_else;
use crate::lang::parser::statement::switch::switch;
use crate::lang::parser::statement::var_assign::var_assign;
use crate::lang::parser::statement::var_def::var_def;
use crate::lang::parser::statement::while_::while_;
use crate::lang::span::{Span, Spanned};

pub mod include;
pub mod var_def;
pub mod var_assign;
pub mod body;
pub mod if_else;
pub mod while_;
pub mod for_;
pub mod switch;

pub fn statement<'tokens>(
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    recursive(|statement| {
        choice((
            include(),
            var_def(),
            var_assign(),
            if_else(statement.clone()),
            while_(statement.clone()),
            for_(statement.clone()),
            switch(statement.clone()),
        ))
    })
}