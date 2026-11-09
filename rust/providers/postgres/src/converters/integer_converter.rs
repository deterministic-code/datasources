use datasources::{DatasourceError, Result, TypeConverter, Value};

pub struct IntegerConverter;

impl TypeConverter<Value, Value> for IntegerConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        match from {
            Value::Text(text) => text.parse::<i64>().map(Value::Int).map_err(|_| {
                DatasourceError::message(format!("\"{text}\" is not a long"))
            }),
            other => Ok(other.clone()),
        }
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(to.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_integer() {
        let converter = IntegerConverter;
        assert_eq!(converter.convert(&Value::from("10")).unwrap(), Value::Int(10));
        assert_eq!(converter.convert(&Value::from("0")).unwrap(), Value::Int(0));
        assert!(converter.convert(&Value::from("")).is_err());
        assert_eq!(converter.convert(&Value::Int(7)).unwrap(), Value::Int(7));
        assert_eq!(converter.reverse_convert(&Value::Int(7)).unwrap(), Value::Int(7));
    }
}
