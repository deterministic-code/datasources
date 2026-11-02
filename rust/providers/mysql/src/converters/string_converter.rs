use datasources::{Result, TypeConverter, Value};

pub struct StringConverter;

impl TypeConverter<Value, Value> for StringConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Text(text) => Value::Text(text.clone()),
            Value::Int(n) => Value::Text(n.to_string()),
            Value::Uuid(id) => Value::Text(id.to_string()),
            Value::Decimal(number) => Value::Text(number.to_string()),
            Value::DateTime(date) => Value::Text(Value::iso_datetime(*date)),
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
    use uuid::Uuid;

    #[test]
    fn maps_string() {
        let converter = StringConverter;
        let id = Uuid::parse_str("54ef41f2-0cb4-4c72-97a7-776fd183f5ed").unwrap();
        assert_eq!(
            converter.convert(&Value::Uuid(id)).unwrap(),
            Value::from(id.to_string())
        );
        assert_eq!(converter.convert(&Value::Int(1)).unwrap(), Value::from("1"));
        assert_eq!(
            converter.convert(&Value::from("ada")).unwrap(),
            Value::from("ada")
        );
    }
}
