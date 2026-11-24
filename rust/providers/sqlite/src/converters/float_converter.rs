use datasources::{Result, TypeConverter, Value};
use rust_decimal::prelude::ToPrimitive;

pub struct FloatConverter;

impl TypeConverter<Value, Value> for FloatConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(number(from))
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(to.clone())
    }
}

fn number(value: &Value) -> Value {
    match value {
        Value::Int(n) => Value::Float(*n as f64),
        Value::Float(n) => Value::Float(*n),
        Value::Decimal(n) => n.to_f64().map(Value::Float).unwrap_or_else(|| value.clone()),
        Value::Text(text) => text
            .parse::<f64>()
            .ok()
            .filter(|n| n.is_finite())
            .map(Value::Float)
            .unwrap_or_else(|| value.clone()),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn maps_float() {
        let converter = FloatConverter;
        assert_eq!(converter.convert(&Value::Int(10)).unwrap(), Value::Float(10.0));
        assert_eq!(converter.convert(&Value::Float(10.9)).unwrap(), Value::Float(10.9));
        let decimal = "1.23456789012346".parse::<Decimal>().unwrap();
        assert_eq!(
            converter.convert(&Value::Decimal(decimal)).unwrap(),
            Value::Float(decimal.to_f64().unwrap())
        );
        assert_eq!(
            converter.convert(&Value::from("1.23456789012346")).unwrap(),
            Value::Float(1.23456789012346)
        );
        assert_eq!(converter.convert(&Value::from("nope")).unwrap(), Value::from("nope"));
        assert_eq!(converter.reverse_convert(&Value::Float(10.9)).unwrap(), Value::Float(10.9));
    }
}
