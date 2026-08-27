use datasources::{DatasourceError, Record, Result, Value};
use serde_json::{Map, Value as JsonValue};

pub(crate) fn parse_filter(text: Option<&str>) -> Result<Map<String, JsonValue>> {
    match text {
        None => Ok(Map::new()),
        Some(value) if value.trim().is_empty() => Ok(Map::new()),
        Some(value) => {
            let parsed: JsonValue = serde_json::from_str(value)
                .map_err(|e| DatasourceError::message(e.to_string()))?;
            match parsed {
                JsonValue::Object(map) => Ok(map),
                _ => Err(DatasourceError::message("JSON filter must be an object")),
            }
        }
    }
}

pub(crate) fn matches(obj: &JsonValue, filter: &Map<String, JsonValue>) -> bool {
    let JsonValue::Object(map) = obj else {
        return false;
    };
    filter.iter().all(|(k, v)| map.get(k) == Some(v))
}

pub(crate) fn json_from_record(record: &Record) -> JsonValue {
    let mut map = Map::new();
    for (k, v) in record {
        map.insert(k.clone(), value_to_json(v));
    }
    JsonValue::Object(map)
}

pub(crate) fn record_from_json(value: &JsonValue) -> Record {
    let mut record = Record::new();
    if let JsonValue::Object(map) = value {
        for (k, v) in map {
            record.insert(k.clone(), json_to_value(v));
        }
    }
    record
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

pub(crate) fn value_to_json(value: &Value) -> JsonValue {
    match value {
        Value::Null => JsonValue::Null,
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Int(i) => JsonValue::from(*i),
        Value::Float(f) => JsonValue::from(*f),
        Value::Text(t) => JsonValue::String(t.clone()),
    }
}
