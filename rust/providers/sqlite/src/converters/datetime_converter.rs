use chrono::{DateTime, Utc};
use datasources::{Result, TypeConverter, Value};

pub struct DateTimeConverter;

impl TypeConverter<Value, Value> for DateTimeConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Text(text) => DateTime::parse_from_rfc3339(text)
                .map(|parsed| Value::DateTime(parsed.with_timezone(&Utc)))
                .unwrap_or_else(|_| from.clone()),
            other => other.clone(),
        })
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        Ok(match to {
            Value::DateTime(date) => Value::Text(Value::iso_datetime(*date)),
            other => other.clone(),
        })
    }
}
