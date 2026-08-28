use crate::converters::fail::fail;
use crate::converters::field_converter::FieldConverter;
use crate::result::Result;
use crate::value::Value;

pub fn uuid_field_converter() -> FieldConverter {
    FieldConverter::new(parse_uuid, parse_uuid)
}

fn parse_uuid(value: &Value) -> Result<Value> {
    match value {
        Value::Null => Ok(Value::Null),
        Value::Text(text) if is_uuid(text) => Ok(Value::Text(text.clone())),
        other => Err(fail(other, "UUID")),
    }
}

pub(crate) fn is_uuid(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 36
        && bytes[8] == b'-'
        && bytes[13] == b'-'
        && bytes[18] == b'-'
        && bytes[23] == b'-'
        && text.chars().enumerate().all(|(i, ch)| match i {
            8 | 13 | 18 | 23 => ch == '-',
            _ => ch.is_ascii_hexdigit(),
        })
}
