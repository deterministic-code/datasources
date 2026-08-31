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
    fn registers_and_converts_sqlite_wire() {
        register();
        assert!(DefaultTypeConverters::for_provider("sqlite").contains_key("bool"));
        let bools = BoolConverter;
        assert_eq!(bools.convert(&Value::Int(1)).unwrap(), Value::Bool(true));
        assert_eq!(
            bools.reverse_convert(&Value::Bool(false)).unwrap(),
            Value::Int(0)
        );
        assert!(matches!(
            DateTimeConverter
                .convert(&Value::from("2020-01-02T03:04:05.000Z"))
                .unwrap(),
            Value::DateTime(_)
        ));
        assert!(matches!(
            UuidConverter
                .convert(&Value::from("550e8400-e29b-41d4-a716-446655440000"))
                .unwrap(),
            Value::Uuid(_)
        ));
        assert_eq!(
            DecimalConverter.convert(&Value::from("10.5")).unwrap(),
            Value::Decimal("10.5".parse().unwrap())
        );
    }
}
