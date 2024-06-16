use crate::parsing::span::{Span, Spanned};

#[derive(Clone, Debug)]
pub enum XSError {
    TypeError(Spanned<String>),
    NameError(Spanned<String>),
    SyntaxError(Spanned<String>),
    
    Warning(Spanned<String>),
}

pub fn type_err(msg: &str, span: &Span) -> XSError {
    XSError::TypeError((
        format!("TypeError: {:}", msg),
        span.clone()
    ))
}
pub fn name_err(msg: &str, span: &Span) -> XSError {
    XSError::NameError((
        format!("NameError: {:}", msg),
        span.clone()
    ))
}
pub fn syntax_err(msg: &str, span: &Span) -> XSError {
    XSError::SyntaxError((
        format!("SyntaxError: {:}", msg),
        span.clone()
    ))
}
pub fn warn(msg: &str, span: &Span) -> XSError {
    XSError::Warning((
        format!("Warning: {:}", msg),
        span.clone()
    ))
}