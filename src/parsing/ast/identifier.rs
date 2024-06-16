use std::hash::{Hash};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier(name.to_string())
    }
}