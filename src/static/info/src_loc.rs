use std::path::PathBuf;
use crate::parsing::span::Span;

#[derive(Clone)]
pub struct SrcLoc {
    file_path: PathBuf,
    span: Span,
}

impl SrcLoc {
    pub fn new(file_path: PathBuf, span: Span) -> Self {
        Self {
            file_path,
            span
        }
    }
}