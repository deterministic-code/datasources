use crate::result::Result;
use crate::value::Value;

#[derive(Clone, Copy, Debug)]
pub struct ValueConverter {
    convert: fn(&Value) -> Result<Value>,
    reverse_convert: fn(&Value) -> Result<Value>,
}

impl ValueConverter {
    pub const fn new(
        convert: fn(&Value) -> Result<Value>,
        reverse_convert: fn(&Value) -> Result<Value>,
    ) -> Self {
        Self {
            convert,
            reverse_convert,
        }
    }

    pub fn convert(&self, value: &Value) -> Result<Value> {
        (self.convert)(value)
    }

    pub fn reverse_convert(&self, value: &Value) -> Result<Value> {
        (self.reverse_convert)(value)
    }
}
