use base64::{engine::general_purpose::STANDARD, Engine};
use datasources::{Result, TypeConverter, Value};

pub struct BinaryConverter;

fn decode(text: &str) -> Option<Vec<u8>> {
    let text = text.trim();
    (text.len() >= 4
        && text.len() % 4 == 0
        && text.contains('=')
        && text
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'=')
        && STANDARD.decode(text).is_ok())
    .then(|| STANDARD.decode(text).ok())
    .flatten()
}

impl TypeConverter<Value, Value> for BinaryConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Text(text) => decode(text)
                .map(Value::Binary)
                .unwrap_or_else(|| from.clone()),
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
    fn maps_binary() {
        let converter = BinaryConverter;
        let raw = Value::Binary(vec![1, 2, 3, 4]);
        assert_eq!(converter.convert(&raw).unwrap(), raw);
        assert_eq!(converter.reverse_convert(&raw).unwrap(), raw);
        assert_eq!(
            converter.reverse_convert(&Value::from("AQIDBA==")).unwrap(),
            raw
        );
        assert_eq!(converter.convert(&Value::from("x")).unwrap(), Value::from("x"));
        assert!(matches!(
            converter.convert(&Value::from("AA==")).unwrap(),
            Value::Binary(_)
        ));
    }
}
