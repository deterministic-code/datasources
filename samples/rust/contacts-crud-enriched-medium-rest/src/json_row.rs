use datasources::{encode_rows, Row, Value};
use serde_json::{Map, Number, Value as Json};

const LISTS: [&str; 3] = ["addresses", "phones", "members"];
const MAPS: [&str; 2] = ["settings", "prefs"];

pub fn json_to_row(value: &Json) -> Result<Row, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "invalid json".to_string())?;
    Ok(object
        .iter()
        .map(|(key, cell)| {
            (
                key.clone(),
                if LISTS.contains(&key.as_str()) {
                    encode_rows(&array_rows(cell))
                } else if MAPS.contains(&key.as_str()) {
                    encode_rows(&map_rows(cell))
                } else {
                    json_value(cell)
                },
            )
        })
        .collect())
}

pub fn row_to_json(row: &Row) -> Json {
    Json::Object(
        row.iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    if LISTS.contains(&key.as_str()) {
                        Json::Array(
                            decode_rows(Some(value))
                                .iter()
                                .map(row_to_json)
                                .collect(),
                        )
                    } else if MAPS.contains(&key.as_str()) {
                        rows_to_map(value)
                    } else {
                        value_json(value)
                    },
                )
            })
            .collect(),
    )
}

pub fn rows_to_json(rows: &[Row]) -> Json {
    Json::Array(rows.iter().map(row_to_json).collect())
}

fn map_rows(value: &Json) -> Vec<Row> {
    value
        .as_object()
        .map(|items| {
            items
                .iter()
                .map(|(key, cell)| {
                    let mut row = if cell.is_object() {
                        json_to_row(cell).unwrap_or_default()
                    } else {
                        let mut row = Row::new();
                        row.insert("value".into(), json_value(cell));
                        row
                    };
                    row.insert("key".into(), Value::Text(key.clone()));
                    row
                })
                .collect()
        })
        .unwrap_or_default()
}

fn rows_to_map(value: &Value) -> Json {
    Json::Object(
        decode_rows(Some(value))
            .into_iter()
            .filter_map(|row| {
                let key = text(row.get("key"));
                if key.is_empty() {
                    return None;
                }
                let rest: Map<String, Json> = row
                    .iter()
                    .filter(|(name, _)| {
                        !matches!(
                            name.as_str(),
                            "key" | "contact_id" | "id" | "uuid" | "created" | "updated" | "version"
                        )
                    })
                    .map(|(name, cell)| (name.clone(), value_json(cell)))
                    .collect();
                Some((
                    key,
                    if rest.len() == 1 {
                        rest.get("value").cloned().unwrap_or(Json::Object(rest))
                    } else {
                        Json::Object(rest)
                    },
                ))
            })
            .collect(),
    )
}

fn array_rows(value: &Json) -> Vec<Row> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| json_to_row(item).ok())
                .collect()
        })
        .unwrap_or_default()
}

fn json_value(value: &Json) -> Value {
    match value {
        Json::Null => Value::Null,
        Json::Bool(flag) => Value::Bool(*flag),
        Json::Number(number) => number
            .as_i64()
            .map(Value::Int)
            .or_else(|| number.as_f64().map(Value::Float))
            .unwrap_or(Value::Null),
        Json::String(text) => Value::Text(text.clone()),
        Json::Array(_) | Json::Object(_) => Value::Text(value.to_string()),
    }
}

fn value_json(value: &Value) -> Json {
    match value {
        Value::Null => Json::Null,
        Value::Bool(flag) => Json::Bool(*flag),
        Value::Int(number) => Json::Number((*number).into()),
        Value::Float(number) => Number::from_f64(*number)
            .map(Json::Number)
            .unwrap_or(Json::Null),
        Value::Text(text) => Json::String(text.clone()),
        Value::DateTime(date) => Json::String(Value::iso_datetime(*date)),
        Value::Uuid(id) => Json::String(id.to_string()),
        Value::Decimal(number) => Json::String(number.to_string()),
        Value::Binary(bytes) => Json::String(
            // version cells match the TS host ("AA==")
            {
                use datasources::Value as V;
                V::Binary(bytes.clone()).wire_text()
            },
        ),
    }
}

fn decode_rows(value: Option<&Value>) -> Vec<Row> {
    match value {
        Some(Value::Text(text)) if !text.is_empty() => text.split('|').map(decode_row).collect(),
        _ => Vec::new(),
    }
}

fn decode_row(text: &str) -> Row {
    text.split(',')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((key.to_string(), parse_value(&unescape(value))))
        })
        .collect()
}

fn unescape(text: &str) -> String {
    text.replace("%7C", "|")
        .replace("%2C", ",")
        .replace("%3D", "=")
        .replace("%25", "%")
}

fn parse_value(text: &str) -> Value {
    match text {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        _ => text
            .parse::<i64>()
            .map(Value::Int)
            .unwrap_or_else(|_| Value::Text(text.into())),
    }
}

pub fn text(value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Null) => String::new(),
        Some(other) => other.wire_text(),
    }
}

pub fn set_text(row: &mut Row, key: &str, fallback: impl Into<String>) {
    let value = text(row.get(key));
    row.insert(key.into(), Value::Text(if value.is_empty() { fallback.into() } else { value }));
}

pub fn ensure_text(row: &mut Row, key: &str) {
    let value = text(row.get(key));
    row.insert(key.into(), Value::Text(value));
}

pub fn snake_key(key: &str) -> String {
    let mut out = String::new();
    for (index, ch) in key.chars().enumerate() {
        if ch.is_uppercase() {
            if index > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn snake_keys(value: Json) -> Json {
    match value {
        Json::Object(map) => Json::Object(
            map.into_iter()
                .map(|(key, cell)| (snake_key(&key), snake_keys(cell)))
                .collect(),
        ),
        Json::Array(items) => Json::Array(items.into_iter().map(snake_keys).collect()),
        other => other,
    }
}

pub fn object_from(body: Option<&str>) -> Result<Json, String> {
    match body.map(str::trim).filter(|text| !text.is_empty()) {
        None => Ok(Json::Object(Map::new())),
        Some(text) => {
            let parsed: Json = serde_json::from_str(text).map_err(|error| error.to_string())?;
            match parsed {
                Json::Object(_) => Ok(snake_keys(parsed)),
                _ => Err("invalid json".into()),
            }
        }
    }
}
