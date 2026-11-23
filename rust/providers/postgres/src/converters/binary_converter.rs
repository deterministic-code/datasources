use base64::{engine::general_purpose::STANDARD, Engine};
use datasources::{Result, TypeConverter, Value};

pub struct BinaryConverter;

impl BinaryConverter {
    pub fn is_base64(text: &str) -> bool {
        decode(text).is_some()
    }
}

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

fn hex(text: &str) -> Option<Vec<u8>> {
    let text = text.trim();
    let digits = text.strip_prefix("\\x").or_else(|| text.strip_prefix("\\X"))?;
    if digits.len() % 2 != 0 {
        return None;
    }
    (0..digits.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&digits[i..i + 2], 16).ok())
        .collect()
}

impl TypeConverter<Value, Value> for BinaryConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Text(text) => decode(text)
                .or_else(|| hex(text))
                .map(Value::Binary)
                .unwrap_or_else(|| Value::Binary(text.as_bytes().to_vec())),
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
        assert_eq!(
            converter.convert(&Value::from("x")).unwrap(),
            Value::Binary(vec![b'x'])
        );
        assert_eq!(
            converter.convert(&Value::from("\u{1}\u{2}\u{3}\u{4}")).unwrap(),
            raw
        );
        assert_eq!(
            converter.convert(&Value::from("\\x01020304")).unwrap(),
            raw
        );
        assert!(matches!(
            converter.convert(&Value::from("AA==")).unwrap(),
            Value::Binary(_)
        ));
    }
}
