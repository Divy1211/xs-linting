use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

// all identifiers are equal for the parser.
impl Hash for Identifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(5);
        state.finish();
    }
}

impl PartialEq for Identifier {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Identifier {}
