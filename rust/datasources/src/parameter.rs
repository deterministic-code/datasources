use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter<T = Value> {
    pub name: String,
    pub value: T,
}

impl Parameter<Value> {
    pub fn new(name: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

impl<T> Parameter<T> {
    pub fn with(name: impl Into<String>, value: T) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}
