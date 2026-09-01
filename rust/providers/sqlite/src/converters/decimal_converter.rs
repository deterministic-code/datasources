use datasources::{Result, TypeConverter, Value};
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct DecimalConverter;

impl TypeConverter<Value, Value> for DecimalConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Int(number) => Value::Decimal(Decimal::from(*number)),
            Value::Text(text) => Decimal::from_str(text)
                .map(Value::Decimal)
                .unwrap_or_else(|_| from.clone()),
            other => other.clone(),
        })
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(match to {
            Value::Decimal(number) => Value::Text(number.to_string()),
            other => other.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn maps_decimal() {
        let converter = DecimalConverter;
        assert_eq!(
            converter.convert(&Value::Int(7)).unwrap(),
            Value::Decimal(7.into())
        );
        assert_eq!(
            converter.convert(&Value::from("10.5")).unwrap(),
            Value::Decimal("10.5".parse().unwrap())
        );
        assert_eq!(
            converter
                .reverse_convert(&Value::Decimal("10.5".parse().unwrap()))
                .unwrap(),
            Value::from("10.5")
        );
        assert_eq!(
            converter.convert(&Value::from("nope")).unwrap(),
            Value::from("nope")
        );
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
