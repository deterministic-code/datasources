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
        self.convert(to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn maps_uuid() {
        let converter = UuidConverter;
        let native = converter
            .convert(&Value::from("550e8400-e29b-41d4-a716-446655440000"))
            .unwrap();
        assert!(matches!(native, Value::Uuid(_)));
        assert_eq!(converter.reverse_convert(&native).unwrap(), native);
        assert_eq!(
            converter
                .reverse_convert(&Value::from("550e8400-e29b-41d4-a716-446655440000"))
                .unwrap(),
            native
        );
        assert_eq!(
            converter.convert(&Value::from("nope")).unwrap(),
            Value::from("nope")
        );
        assert_eq!(converter.convert(&Value::Int(1)).unwrap(), Value::Int(1));
        assert_eq!(
            converter.reverse_convert(&Value::Int(1)).unwrap(),
            Value::Int(1)
        );
    }
}
