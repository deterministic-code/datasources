use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub result: Value,
    pub rows_affected: i64,
}

impl Response {
    pub fn new(result: impl Into<Value>, rows_affected: i64) -> Self {
        Self {
            result: result.into(),
            rows_affected,
        }
    }
}
