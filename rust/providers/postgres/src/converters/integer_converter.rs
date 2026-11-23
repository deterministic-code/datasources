use datasources::{DatasourceError, Result, TypeConverter, Value};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub struct IntegerConverter;

fn integral_decimal(number: Decimal) -> Result<Value> {
    match number.to_i64() {
        Some(value) if number.fract().is_zero() => Ok(Value::Int(value)),
        _ => Err(DatasourceError::message(format!("\"{number}\" is not a long"))),
    }
}

fn parse(text: &str) -> Result<Value> {
    if let Ok(number) = text.parse::<i64>() {
        return Ok(Value::Int(number));
    }
    let number: f64 = text
        .parse()
        .map_err(|_| DatasourceError::message(format!("\"{text}\" is not a long")))?;
    if number.fract() == 0.0 && (i64::MIN as f64..=i64::MAX as f64).contains(&number) {
        return Ok(Value::Int(number as i64));
    }
    Err(DatasourceError::message(format!("\"{text}\" is not a long")))
}

impl TypeConverter<Value, Value> for IntegerConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        match from {
            Value::Text(text) => parse(text),
            Value::Float(n)
                if n.is_finite()
                    && n.fract() == 0.0
                    && (i64::MIN as f64..=i64::MAX as f64).contains(n) =>
            {
                Ok(Value::Int(*n as i64))
            }
            Value::Decimal(number) => integral_decimal(*number),
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
        assert_eq!(converter.convert(&Value::from("1.0")).unwrap(), Value::Int(1));
        assert_eq!(converter.convert(&Value::from("10.0")).unwrap(), Value::Int(10));
        assert_eq!(
            converter.convert(&Value::from("10.00000000000000")).unwrap(),
            Value::Int(10)
        );
        assert!(converter.convert(&Value::from("10.5")).is_err());
        assert!(converter.convert(&Value::from("10.50000000000000")).is_err());
        assert_eq!(
            converter.convert(&Value::Float(10.0)).unwrap(),
            Value::Int(10)
        );
        assert_eq!(converter.convert(&Value::Int(7)).unwrap(), Value::Int(7));
        assert_eq!(converter.reverse_convert(&Value::Int(7)).unwrap(), Value::Int(7));
    }
}
