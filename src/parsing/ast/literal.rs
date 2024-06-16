use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Int(_), Literal::Int(_))       => true,
            (Literal::Float(_), Literal::Float(_))   => true,
            (Literal::Bool(_), Literal::Bool(_))     => true,
            (Literal::Str(_), Literal::Str(_))       => true,
            _                                        => false,
        }
    }
}

impl Eq for Literal {}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(match self {
            Literal::Int(_) =>     0,
            Literal::Float(_) =>   1,
            Literal::Bool(_) =>    2,
            Literal::Str(_) =>     3,
        });
        state.finish();
    }
}

impl Literal {
    pub fn str(val: &str) -> Self {
        Literal::Str(val.to_string())
    }
}