use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function {
        name: String,
        params: Vec<String>,
        body: String,
    },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Boolean(true) => write!(f, "fax"),
            Value::Boolean(false) => write!(f, "cap"),
            Value::Null => write!(f, "nah"),
            Value::Function { name, .. } => write!(f, "<fun {name}>"),
        }
    }
}
