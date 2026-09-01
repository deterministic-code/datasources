use datasources::{Result, TypeConverter, Value};

pub struct BinaryConverter;

impl TypeConverter<Value, Value> for BinaryConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(from.clone())
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
    fn maps_binary() {
        let converter = BinaryConverter;
        let bytes = Value::Binary(vec![1, 2]);
        assert_eq!(converter.convert(&bytes).unwrap(), bytes);
        assert_eq!(converter.reverse_convert(&bytes).unwrap(), bytes);
        assert_eq!(
            converter.convert(&Value::from("x")).unwrap(),
            Value::from("x")
        );
        assert_eq!(
            converter.reverse_convert(&Value::from("x")).unwrap(),
            Value::from("x")
        );
    }
}
