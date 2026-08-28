use crate::converters::fail::fail;
use crate::converters::field_converter::FieldConverter;
use crate::datasource_error::DatasourceError;
use crate::result::Result;
use crate::value::Value;

pub fn date_time_field_converter() -> FieldConverter {
    FieldConverter::new(
        |value| match value {
            Value::Null => Ok(Value::Null),
            Value::Text(text) => Ok(Value::Text(parse_iso(text)?)),
            other => Err(fail(other, "DateTime")),
        },
        |value| match value {
            Value::Null => Ok(Value::Null),
            Value::Text(text) => Ok(Value::Text(parse_iso(text)?)),
            other => Err(fail(other, "ISO 8601")),
        },
    )
}

fn parse_iso(text: &str) -> Result<String> {
    if text.len() >= 19
        && text.as_bytes().get(4) == Some(&b'-')
        && text.as_bytes().get(10) == Some(&b'T')
    {
        return Ok(text.to_string());
    }
    Err(DatasourceError::message(format!(
        "Cannot convert \"{text}\" to Date"
    )))
}
