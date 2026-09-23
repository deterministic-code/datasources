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

pub fn row_type_converters() -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>
{
    ["bool", "datetime", "uuid", "decimal", "binary"]
        .into_iter()
        .map(|key| (key.into(), field(key).unwrap_or_else(|| panic!("{key}"))))
        .collect()
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
}
