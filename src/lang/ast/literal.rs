#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Vec {x: f64, y: f64, z: f64},
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub struct Comment(pub String);
