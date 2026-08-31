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
