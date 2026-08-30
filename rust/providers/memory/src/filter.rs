use datasources::{DatasourceError, Row, Result, Value};
use serde_json::Value as JsonValue;

pub(crate) fn parse_filter(text: Option<&str>) -> Result<Row> {
    match text {
        None => Ok(Row::new()),
        Some(value) if value.trim().is_empty() => Ok(Row::new()),
        Some(value) => {
            let parsed: JsonValue = serde_json::from_str(value)
                .map_err(|e| DatasourceError::message(e.to_string()))?;
            match parsed {
                JsonValue::Object(map) => Ok(map
                    .into_iter()
                    .map(|(k, v)| (k, json_to_value(&v)))
                    .collect()),
                _ => Err(DatasourceError::message("Memory filter must be an object")),
            }
        }
    }
}

pub(crate) fn matches(row: &Row, filter: &Row) -> bool {
    filter.iter().all(|(k, v)| row.get(k) == Some(v))
}

pub(crate) fn json_to_value(v: &JsonValue) -> Value {
    match v {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(b) => Value::Bool(*b),
        JsonValue::Number(n) if n.is_i64() => Value::Int(n.as_i64().unwrap()),
        JsonValue::Number(n) if n.is_f64() => Value::Float(n.as_f64().unwrap()),
        JsonValue::Number(n) => Value::Text(n.to_string()),
        JsonValue::String(s) => Value::Text(s.clone()),
        other => Value::Text(other.to_string()),
    }
}
