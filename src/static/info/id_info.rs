use crate::parsing::ast::Type;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Clone)]
pub struct IdInfo {
    pub type_: Type,
    pub src_loc: SrcLoc,
}

impl IdInfo {
    pub fn from(type_: &Type, src_loc: SrcLoc) -> Self {
        Self { type_: type_.clone(), src_loc }
    }

    pub fn new(type_: Type, src_loc: SrcLoc) -> Self {
        Self { type_, src_loc }
    }
    
    pub fn dummy(type_: Type) -> Self {
        Self { type_, src_loc: Default::default() }
    }
}