mod binary_converter;
mod bool_converter;
mod datetime_converter;
mod decimal_converter;
mod uuid_converter;

use datasources::{DefaultTypeConverters, TypeConverter, Value};
use std::collections::HashMap;

pub use binary_converter::BinaryConverter;
pub use bool_converter::BoolConverter;
pub use datetime_converter::DateTimeConverter;
pub use decimal_converter::DecimalConverter;
pub use uuid_converter::UuidConverter;

pub fn types() -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    HashMap::from([
        (
            "bool".into(),
            Box::new(BoolConverter) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        ),
        (
            "datetime".into(),
            Box::new(DateTimeConverter) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        ),
        (
            "uuid".into(),
            Box::new(UuidConverter) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        ),
        (
            "decimal".into(),
            Box::new(DecimalConverter) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        ),
        (
            "binary".into(),
            Box::new(BinaryConverter) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
        ),
    ])
}

pub fn field(canonical: &str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    match canonical {
        "bool" => Some(Box::new(BoolConverter)),
        "datetime" => Some(Box::new(DateTimeConverter)),
        "uuid" => Some(Box::new(UuidConverter)),
        "decimal" => Some(Box::new(DecimalConverter)),
        "binary" => Some(Box::new(BinaryConverter)),
        _ => None,
    }
}

pub fn register() {
    DefaultTypeConverters::use_provider("sqlite", types);
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn registers_catalog() {
        register();
        let types = DefaultTypeConverters::for_provider("sqlite");
        for key in ["bool", "datetime", "uuid", "decimal", "binary"] {
            assert!(types.contains_key(key));
            assert!(field(key).is_some());
        }
        assert!(field("int").is_none());
        assert!(DefaultTypeConverters::for_provider("postgres").is_empty());
    }

    #[test]
    fn bool_converter() {
        let converter = BoolConverter;
        assert_eq!(converter.convert(&Value::Int(1)).unwrap(), Value::Bool(true));
        assert_eq!(converter.convert(&Value::Int(0)).unwrap(), Value::Bool(false));
        assert_eq!(
            converter.convert(&Value::from("x")).unwrap(),
            Value::from("x")
        );
        assert_eq!(
            converter.reverse_convert(&Value::Bool(true)).unwrap(),
            Value::Int(1)
        );
        assert_eq!(
            converter.reverse_convert(&Value::Bool(false)).unwrap(),
            Value::Int(0)
        );
        assert_eq!(
            converter.reverse_convert(&Value::from("x")).unwrap(),
            Value::from("x")
        );
    }

    #[test]
    fn datetime_converter() {
        let converter = DateTimeConverter;
        let native = converter
            .convert(&Value::from("2020-01-02T03:04:05.000Z"))
            .unwrap();
        assert!(matches!(native, Value::DateTime(_)));
        assert_eq!(
            converter.reverse_convert(&native).unwrap(),
            Value::from("2020-01-02T03:04:05.000Z")
        );
        assert_eq!(
            converter.convert(&Value::from("nope")).unwrap(),
            Value::from("nope")
        );
        assert_eq!(converter.convert(&Value::Int(1)).unwrap(), Value::Int(1));
        assert_eq!(
            converter.reverse_convert(&Value::Int(1)).unwrap(),
            Value::Int(1)
        );
    }

    #[test]
    fn uuid_converter() {
        let converter = UuidConverter;
        let native = converter
            .convert(&Value::from("550e8400-e29b-41d4-a716-446655440000"))
            .unwrap();
        assert!(matches!(native, Value::Uuid(_)));
        assert_eq!(
            converter.reverse_convert(&native).unwrap(),
            Value::from("550e8400-e29b-41d4-a716-446655440000")
        );
        assert_eq!(
            converter.convert(&Value::from("nope")).unwrap(),
            Value::from("nope")
        );
        assert_eq!(converter.convert(&Value::Int(1)).unwrap(), Value::Int(1));
        assert_eq!(
            converter.reverse_convert(&Value::Int(1)).unwrap(),
            Value::Int(1)
        );
    }

    #[test]
    fn decimal_converter() {
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

    #[test]
    fn binary_converter() {
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
