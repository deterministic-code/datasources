use chrono::{DateTime, NaiveDateTime, Utc};
use datasources::{Result, TypeConverter, Value};

pub struct DateTimeConverter;

fn parse_text(text: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(text)
        .ok()
        .map(|parsed| parsed.with_timezone(&Utc))
        .or_else(|| {
            let iso = text.replace(' ', "T");
            let padded = if iso.ends_with("+00") || iso.ends_with("-00") {
                format!("{iso}:00")
            } else {
                iso
            };
            DateTime::parse_from_rfc3339(&padded)
                .ok()
                .map(|parsed| parsed.with_timezone(&Utc))
        })
        .or_else(|| {
            NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S%.f")
                .or_else(|_| NaiveDateTime::parse_from_str(text, "%Y-%m-%d %H:%M:%S"))
                .ok()
                .map(|naive| naive.and_utc())
        })
}

impl TypeConverter<Value, Value> for DateTimeConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        Ok(match from {
            Value::Text(text) => parse_text(text)
                .map(Value::DateTime)
                .unwrap_or_else(|| from.clone()),
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
            Value::from("2020-01-02T03:04:05.000Z")
        );
        assert_eq!(
            converter
                .convert(&Value::from("2020-01-02 03:04:05+00"))
                .unwrap(),
            native
        );
        assert_eq!(
            converter
                .convert(&Value::from("2020-01-02 03:04:05.000"))
                .unwrap(),
            native
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
