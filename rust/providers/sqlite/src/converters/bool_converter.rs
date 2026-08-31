use datasources::{Result, TypeConverter, Value};

pub struct BoolConverter;

impl TypeConverter<Value, Value> for BoolConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Int(number) => Value::Bool(*number != 0),
            other => other.clone(),
        })
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(match to {
            Value::Bool(flag) => Value::Int(if *flag { 1 } else { 0 }),
            other => other.clone(),
        })
    }
}
