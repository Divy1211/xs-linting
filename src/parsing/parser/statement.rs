use chumsky::prelude::*;

use crate::parsing::ast::astree::ASTreeNode;
use crate::parsing::lexer::token::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::fn_def::fn_def;
use crate::parsing::parser::statement::for_::for_;
use crate::parsing::parser::statement::include::include;
use crate::parsing::parser::statement::if_else::if_else;
use crate::parsing::parser::statement::break_or_continue::break_or_continue_or_breakpt;
use crate::parsing::parser::statement::class_def::class_def;
use crate::parsing::parser::statement::discarded_expr::discarded_expr;
use crate::parsing::parser::statement::label_def_or_goto_or_dbg::label_def_or_goto_or_dbg;
use crate::parsing::parser::statement::postfix::postfix;
use crate::parsing::parser::statement::return_::return_;
use crate::parsing::parser::statement::rule_def::rule_def;
use crate::parsing::parser::statement::switch::switch;
use crate::parsing::parser::statement::var_assign::var_assign;
use crate::parsing::parser::statement::var_def::var_def;
use crate::parsing::parser::statement::while_::while_;
use crate::parsing::span::{Span, Spanned};

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