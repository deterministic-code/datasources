use datasources::{Result, TypeConverter, Value};
use uuid::Uuid;

pub struct UuidConverter;

impl TypeConverter<Value, Value> for UuidConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Text(text) => Uuid::parse_str(text)
                .map(Value::Uuid)
                .unwrap_or_else(|_| from.clone()),
            other => other.clone(),
        })
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(match to {
            Value::Uuid(id) => Value::Text(id.to_string()),
            other => other.clone(),
        })
    }
}
