use chumsky::error::Rich;
use chumsky::prelude::*;

use crate::parsing::lexer::token::Token;
use crate::parsing::span::Span;

pub fn comment<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    let line_comment = just("//").ignore_then(none_of("\r\n").repeated());
    let block_comment = just("/*").ignore_then(none_of("*/").repeated());


    // let block_comment = just("/*").ignore_then(custom::<_, &str, _, _>(|inp| {
    //     let mut fslash_is_end = false;
    //     loop {
    //         match inp.next() {
    //             Some('/')  if fslash_is_end => { return Ok(()); }
    //             Some('*') => { fslash_is_end = true; }
    //             Some(___) => { fslash_is_end = false; }
    //             None => { ??? } // unexpected EoF
    //         }
    //     }
    // }))
    
    line_comment.or(block_comment)
        .to_slice()
        .map(|val: &str| Token::Comment(String::from(val)))
}