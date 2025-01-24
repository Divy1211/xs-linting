use crate::parsing::ast::type_::Type;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Clone)]
pub struct IdInfo {
    pub type_: Type,
    pub src_loc: SrcLoc,
}

impl IdInfo {
    pub fn new(type_: Type, src_loc: SrcLoc) -> Self {
        Self { type_, src_loc }
    }
}