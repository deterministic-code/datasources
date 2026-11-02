use datasources::{Result, TypeConverter, Value};
use rust_decimal::Decimal;
use std::str::FromStr;

pub struct DecimalConverter;

impl TypeConverter<Value, Value> for DecimalConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(canonical(from))
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(canonical(to))
    }
}

fn canonical(value: &Value) -> Value {
    match value {
        Value::Int(number) => text(Decimal::from(*number)),
        Value::Float(number) => Decimal::from_f64_retain(*number)
            .map(text)
            .unwrap_or_else(|| value.clone()),
        Value::Decimal(number) => text(*number),
        Value::Text(body) => Decimal::from_str(body)
            .map(text)
            .unwrap_or_else(|_| value.clone()),
        other => other.clone(),
    }
}

fn text(number: Decimal) -> Value {
    Value::Text(number.normalize().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn maps_decimal() {
        let converter = DecimalConverter;
        assert_eq!(converter.convert(&Value::Int(7)).unwrap(), Value::from("7"));
        assert_eq!(
            converter.convert(&Value::Float(10.5)).unwrap(),
            Value::from("10.5")
        );
        assert_eq!(
            converter.convert(&Value::from("10.5")).unwrap(),
            Value::from("10.5")
        );
        assert_eq!(
            converter.convert(&Value::from("10.50")).unwrap(),
            Value::from("10.5")
        );
        assert_eq!(converter.convert(&Value::Int(0)).unwrap(), Value::from("0"));
        assert_eq!(
            converter.convert(&Value::from("0.00")).unwrap(),
            Value::from("0")
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
