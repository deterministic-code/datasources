use datasources::{Result, TypeConverter, Value};
use uuid::Uuid;

pub struct UuidConverter;

impl TypeConverter<Value, Value> for UuidConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(canonical(from))
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(canonical(to))
    }
}

fn canonical(value: &Value) -> Value {
    match value {
        Value::Uuid(id) => Value::Text(id.to_string()),
        Value::Text(text) => Uuid::parse_str(text)
            .map(|id| Value::Text(id.to_string()))
            .unwrap_or_else(|_| value.clone()),
        Value::Binary(bytes) => Uuid::from_slice(bytes)
            .map(|id| Value::Text(id.to_string()))
            .unwrap_or_else(|_| value.clone()),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn maps_uuid() {
        let converter = UuidConverter;
        let id = "550e8400-e29b-41d4-a716-446655440000";
        let uuid = Uuid::parse_str(id).unwrap();
        let text = Value::from(id);
        assert_eq!(converter.convert(&Value::from(id)).unwrap(), text);
        assert_eq!(converter.convert(&Value::Uuid(uuid)).unwrap(), text);
        assert_eq!(
            converter
                .convert(&Value::Binary(uuid.as_bytes().to_vec()))
                .unwrap(),
            text
        );
        assert_eq!(converter.reverse_convert(&Value::Uuid(uuid)).unwrap(), text);
        assert_eq!(
            converter.convert(&Value::from("nope")).unwrap(),
            Value::from("nope")
        );
        assert_eq!(converter.convert(&Value::Int(1)).unwrap(), Value::Int(1));
        assert_eq!(
            converter.convert(&Value::Bool(true)).unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            converter.reverse_convert(&Value::Bool(true)).unwrap(),
            Value::Bool(true)
        );
    }
}
