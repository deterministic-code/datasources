use chrono::{DateTime, Utc};
use datasources::{Result, TypeConverter, Value};

fn stamp(date: DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

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
            Value::DateTime(date) => Value::Text(stamp(*date)),
            Value::Text(text) if text.contains('T') => DateTime::parse_from_rfc3339(text)
                .map(|parsed| Value::Text(stamp(parsed.with_timezone(&Utc))))
                .unwrap_or_else(|_| to.clone()),
            other => other.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datasources::Value;

    #[test]
    fn maps_datetime() {
        let converter = DateTimeConverter;
        let native = converter
            .convert(&Value::from("2020-01-02T03:04:05.000Z"))
            .unwrap();
        assert!(matches!(native, Value::DateTime(_)));
        assert_eq!(
            converter.reverse_convert(&native).unwrap(),
            Value::from("2020-01-02 03:04:05.000")
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
}
