use crate::parsing::ast::identifier::Identifier;
use crate::r#static::info::id_info::IdInfo;
use crate::r#static::info::src_loc::SrcLoc;
use std::collections::HashMap;

pub struct FnInfo {
    identifiers: HashMap<Identifier, IdInfo>,
    src_loc: SrcLoc
}

impl FnInfo {
    pub fn new(identifiers: HashMap<Identifier, IdInfo>, src_loc: SrcLoc) -> Self {
        Self { identifiers, src_loc }
    }
    
    pub fn get(&self, id: &Identifier) -> Option<&IdInfo> {
        self.identifiers.get(id)
    }
    
    pub fn set(&mut self, id: Identifier, info: IdInfo) {
        self.identifiers.insert(id, info);
    }
}