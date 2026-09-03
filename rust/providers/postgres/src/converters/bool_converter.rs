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
        Ok(to.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn maps_bool() {
        let converter = BoolConverter;
        assert_eq!(
            converter.convert(&Value::Int(1)).unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            converter.convert(&Value::Int(0)).unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            converter.convert(&Value::from("x")).unwrap(),
            Value::from("x")
        );
        assert_eq!(
            converter.reverse_convert(&Value::Bool(true)).unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            converter.reverse_convert(&Value::Bool(false)).unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            converter.reverse_convert(&Value::from("x")).unwrap(),
            Value::from("x")
        );
    }
}
