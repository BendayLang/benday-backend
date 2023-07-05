use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ReturnValue {
    String_(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    None,
}

impl fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnValue::String_(str) => write!(f, "{str}"),
            ReturnValue::Int(val) => write!(f, "{val}"),
            ReturnValue::Float(val) => write!(f, "{val}"),
            ReturnValue::Bool(val) => write!(f, "{val}"),
            ReturnValue::None => write!(f, "None"),
        }
    }
}
