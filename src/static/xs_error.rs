use crate::parsing::span::{Span};

pub enum XSError {
    // type errors
    ExtraArg { fn_name: String, span: Span },
    TypeMismatch { actual: String, expected: String, span: Span, note: Option<String> },
    NotCallable { name: String, actual: String, span: Span },
    OpMismatch { op: String, type1: String, type2: String, span: Span, note: Option<String> },

    // name errors
    UndefinedName { name: String, span: Span },
    RedefinedName { name: String, span: Span, og_span: Span, note: Option<String> },

    Syntax { span: Span, msg: String, keywords: Vec<String> },

    Warning { span: Span, msg: String, keywords: Vec<String> },
}

impl XSError {
    pub fn extra_arg(fn_name: &str, span: &Span) -> XSError {
        XSError::ExtraArg {
            fn_name: String::from(fn_name),
            span: span.clone(),
        }
    }

    pub fn type_mismatch(actual: &str, expected: &str, span: &Span, note: Option<&str>) -> XSError {
        XSError::TypeMismatch {
            actual: String::from(actual),
            expected: String::from(expected),
            span: span.clone(),
            note: note.map(String::from),
        }
    }
    
    pub fn not_callable(name: &str, actual: &str, span: &Span) -> XSError {
        XSError::NotCallable {
            name: String::from(name),
            actual: String::from(actual),
            span: span.clone(),
        }
    }

    pub fn op_mismatch(op: &str, type1: &str, type2: &str, span: &Span, note: Option<&str>) -> XSError {
        XSError::OpMismatch {
            op: String::from(op),
            type1: String::from(type1),
            type2: String::from(type2),
            span: span.clone(),
            note: note.map(String::from),
        }
    }

    pub fn undefined_name(name: &str, span: &Span) -> XSError {
        XSError::UndefinedName {
            name: String::from(name),
            span: span.clone(),
        }
    }

    pub fn redefined_name(name: &str, span: &Span, og_span: &Span, note: Option<&str>) -> XSError {
        XSError::RedefinedName {
            name: String::from(name),
            span: span.clone(),
            og_span: og_span.clone(),
            note: note.map(String::from),
        }
    }
    
    pub fn syntax(span: &Span, msg: &str, keywords: Vec<&str>) -> XSError {
        XSError::Syntax {
            span: span.clone(),
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
        }
    }

    pub fn warning(span: &Span, msg: &str, keywords: Vec<&str>) -> XSError {
        XSError::Warning {
            span: span.clone(),
            msg: String::from(msg),
            keywords: keywords.into_iter().map(String::from).collect(),
        }
    }
}
