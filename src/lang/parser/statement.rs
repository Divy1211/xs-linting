use chumsky::prelude::*;

use crate::lang::ast::astree::ASTreeNode;
use crate::lang::lexer::token::Token;
use crate::lang::parser::parser_input::ParserInput;
use crate::lang::parser::statement::fn_def::fn_def;
use crate::lang::parser::statement::for_::for_;
use crate::lang::parser::statement::include::include;
use crate::lang::parser::statement::if_else::if_else;
use crate::lang::parser::statement::break_or_continue::break_or_continue_or_breakpt;
use crate::lang::parser::statement::class_def::class_def;
use crate::lang::parser::statement::discarded_expr::discarded_expr;
use crate::lang::parser::statement::label_def_or_goto_or_dbg::label_def_or_goto_or_dbg;
use crate::lang::parser::statement::postfix::postfix;
use crate::lang::parser::statement::return_::return_;
use crate::lang::parser::statement::rule_def::rule_def;
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
pub mod fn_def;
pub mod break_or_continue;
pub mod return_;
pub mod rule_def;
pub mod postfix;
pub mod label_def_or_goto_or_dbg;
pub mod discarded_expr;
pub mod class_def;

pub fn statement<'tokens>() -> impl Parser<
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
            break_or_continue_or_breakpt(),
            fn_def(statement.clone()),
            return_(),
            rule_def(statement.clone()),
            postfix(),
            label_def_or_goto_or_dbg(),
            discarded_expr(),
            class_def(),
        ))
    })
}