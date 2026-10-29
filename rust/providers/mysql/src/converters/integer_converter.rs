use datasources::{Result, TypeConverter, Value};

pub struct IntegerConverter;

impl TypeConverter<Value, Value> for IntegerConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Text(text) if !text.is_empty() => text
                .parse::<i64>()
                .map(Value::Int)
                .unwrap_or_else(|_| from.clone()),
            other => other.clone(),
        })
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(to.clone())
    }
}
