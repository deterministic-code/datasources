mod binary_converter;
mod bool_converter;
mod datetime_converter;
mod decimal_converter;
mod uuid_converter;

use datasources::{TypeConverter, Value};
use std::collections::HashMap;

pub use binary_converter::BinaryConverter;
pub use bool_converter::BoolConverter;
pub use datetime_converter::DateTimeConverter;
pub use decimal_converter::DecimalConverter;
pub use uuid_converter::UuidConverter;

fn catalog() -> Vec<(&'static str, Box<dyn TypeConverter<Value, Value> + Send + Sync>)> {
    vec![
        ("bool", Box::new(BoolConverter)),
        ("datetime", Box::new(DateTimeConverter)),
        ("uuid", Box::new(UuidConverter)),
        ("decimal", Box::new(DecimalConverter)),
        ("binary", Box::new(BinaryConverter)),
    ]
}

pub fn row_type_converters() -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
{
    catalog()
        .into_iter()
        .map(|(key, converter)| (key.to_string(), converter))
        .collect()
}

pub fn field(canonical: &str) -> Option<Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
    catalog()
        .into_iter()
        .find(|(key, _)| *key == canonical)
        .map(|(_, converter)| converter)
}

pub fn convert(value: &Value) -> Value {
    match value {
        Value::Text(_) => field("datetime")
            .and_then(|converter| converter.convert(value).ok())
            .filter(|converted| matches!(converted, Value::DateTime(_)))
            .unwrap_or_else(|| value.clone()),
        _ => value.clone(),
    }
}

pub fn reverse(value: &Value) -> Value {
    let key = match value {
        Value::Bool(_) => "bool",
        Value::DateTime(_) => "datetime",
        Value::Text(text) if text.contains('T') && text.contains('-') => "datetime",
        Value::Uuid(_) => "uuid",
        Value::Decimal(_) => "decimal",
        Value::Binary(_) => "binary",
        _ => return value.clone(),
    };
    field(key)
        .and_then(|converter| converter.reverse_convert(value).ok())
        .unwrap_or_else(|| value.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_catalog() {
        let catalog = row_type_converters();
        for key in ["bool", "datetime", "uuid", "decimal", "binary"] {
            assert!(catalog.contains_key(key));
            assert!(field(key).is_some());
        }
        assert!(field("int").is_none());
    }

    #[test]
    fn row_converter_reverses_uuid_to_text() {
        use datasources::RowTypeConverter;
        use std::collections::HashMap;
        let id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let converter = RowTypeConverter::new(Vec::new()).with_types(row_type_converters());
        let row = HashMap::from([("uuid".into(), Value::Uuid(id))]);
        assert_eq!(
            converter.reverse_convert(&row).unwrap()["uuid"],
            Value::from(id.to_string())
        );
    }
}
