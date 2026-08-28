use crate::converters::fail::fail;
use crate::converters::field_converter::FieldConverter;
use crate::value::Value;

pub fn binary_field_converter() -> FieldConverter {
    FieldConverter::new(
        |value| match value {
            Value::Null => Ok(Value::Null),
            Value::Text(text) => Ok(Value::Text(text.clone())),
            other => Err(fail(other, "binary")),
        },
        |value| match value {
            Value::Null => Ok(Value::Null),
            Value::Text(text) => Ok(Value::Text(text.clone())),
            other => Err(fail(other, "bytes")),
        },
    )
}
