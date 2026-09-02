use base64::{engine::general_purpose::STANDARD, Engine};
use datasources::{Result, TypeConverter, Value};

pub struct BinaryConverter;

fn bytes(value: &Value) -> Value {
    match value {
        Value::Text(text) => STANDARD
            .decode(text)
            .map(Value::Binary)
            .unwrap_or_else(|_| Value::Binary(text.as_bytes().to_vec())),
        other => other.clone(),
    }
}

impl TypeConverter<Value, Value> for BinaryConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(bytes(from))
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(bytes(to))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn maps_binary() {
        let converter = BinaryConverter;
        let raw = Value::Binary(vec![1, 2, 3, 4]);
        assert_eq!(converter.convert(&raw).unwrap(), raw);
        assert_eq!(converter.reverse_convert(&raw).unwrap(), raw);
        assert_eq!(
            converter.reverse_convert(&Value::from("AQIDBA==")).unwrap(),
            raw
        );
        assert_eq!(
            converter.convert(&Value::from("x")).unwrap(),
            Value::Binary(vec![b'x'])
        );
    }
}
