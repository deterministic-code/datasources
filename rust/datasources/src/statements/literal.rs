use crate::value::Value;

#[derive(Clone)]
pub struct Literal {
    pub value: Value,
}

impl Literal {
    pub fn new(value: impl Into<Value>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
