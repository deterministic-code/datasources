use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub value: Value,
}

impl Parameter {
    pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
