use chumsky::input::SpannedInput;
use crate::lang::lexer::token::Token;
use crate::lang::span::{Span, Spanned};

pub type ParserInput<'tokens> = SpannedInput<Token, Span, &'tokens [Spanned<Token>]>;
