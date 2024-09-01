use crate::parsing::span::{Span, Spanned};

// #[derive(Clone, Debug)]
// pub enum XSError {
//     // type errors
//     ExtraArgError { fn_name: String, span: Span },
//     TypeMismatchError { actual: String, expected: String, span: Span, note: Option<String> },
//     CallableError { name: String, actual: String, span: Span },
//     OpError { op: String, type1: String, type2: String, span: Span, note: String },
//
//     // name errors
//     UndefinedError { name: String, span: Span },
//     RedefinedError { name: String, span: Span, original_span: Span, note: Option<String> },
//
//     SyntaxError { span: Span, msg: String, keywords: Vec<String> },
//
//     Warning { span: Span, msg: String, keywords: Vec<String> },
// }

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