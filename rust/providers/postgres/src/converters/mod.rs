mod binary_converter;
mod bool_converter;
mod datetime_converter;
mod decimal_converter;
mod integer_converter;
mod uuid_converter;

use datasources::{TypeConverter, Value};
use std::collections::HashMap;

pub use binary_converter::BinaryConverter;
pub use bool_converter::BoolConverter;
pub use datetime_converter::DateTimeConverter;
pub use decimal_converter::DecimalConverter;
pub use integer_converter::IntegerConverter;
pub use uuid_converter::UuidConverter;

fn catalog() -> Vec<(&'static str, Box<dyn TypeConverter<Value, Value> + Send + Sync>)> {
    vec![
        ("boolean", Box::new(BoolConverter)),
        ("datetime", Box::new(DateTimeConverter)),
        ("uuid", Box::new(UuidConverter)),
        ("decimal", Box::new(DecimalConverter)),
        ("binary", Box::new(BinaryConverter)),
        ("integer", Box::new(IntegerConverter)),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_catalog() {
        let catalog = row_type_converters();
        for key in [
            "boolean",
            "datetime",
            "uuid",
            "decimal",
            "binary",
            "integer",
        ] {
            assert!(catalog.contains_key(key));
            assert!(field(key).is_some());
        }
        assert!(field("int").is_none());
    }

    #[test]
    fn row_converter_reverses_text_uuid() {
        use datasources::RowTypeConverter;
        use std::collections::HashMap;
        let text = "550e8400-e29b-41d4-a716-446655440000";
        let converter = RowTypeConverter::from(
            row_type_converters,
            &HashMap::from([("uuid".into(), "uuid".into())]),
        );
        let row = HashMap::from([("uuid".into(), Value::from(text))]);
        assert!(matches!(
            converter.reverse_convert(&row).unwrap()["uuid"],
            Value::Uuid(_)
        ));
    }
}
