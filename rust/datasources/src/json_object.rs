use crate::converters::{is_column, peek_value, reflect, set_value, struct_fields};
use crate::datasource_error::DatasourceError;
use crate::result::Result;
use crate::value::Value;
use chrono::{DateTime, Utc};
use facet::{Def, Facet, HasFields, Partial, Peek, Shape};
use serde_json::{Map, Number, Value as Json};

pub struct JsonObjectSerializer;

impl JsonObjectSerializer {
    pub fn serialize<T: Facet<'static>>(value: &T) -> Result<Json> {
        peek_to_json(Peek::new(value))
    }

    pub fn deserialize<T: Facet<'static>>(value: &Json) -> Result<T> {
        let partial = write_into(Partial::alloc::<T>().map_err(reflect)?, value, T::SHAPE)?;
        partial.build().map_err(reflect)?.materialize().map_err(reflect)
    }

    pub fn to_string<T: Facet<'static>>(value: &T) -> Result<String> {
        Ok(Self::serialize(value)?.to_string())
    }

    pub fn from_str<T: Facet<'static> + Default>(text: &str) -> Result<T> {
        if text.trim().is_empty() {
            return Ok(T::default());
        }
        let json = serde_json::from_str(text).map_err(|error| DatasourceError::message(error.to_string()))?;
        Self::deserialize(&json)
    }
}

fn peek_to_json(value: Peek<'_, '_>) -> Result<Json> {
    if let Ok(option) = value.into_option() {
        return match option.value() {
            Some(inner) => peek_to_json(inner),
            None => Ok(Json::Null),
        };
    }
    if is_column(value.shape()) {
        return Ok(value_to_json(peek_value(value).unwrap_or(Value::Null)));
    }
    if let Ok(list) = value.into_list() {
        return list.iter().map(peek_to_json).collect::<Result<Vec<_>>>().map(Json::Array);
    }
    if let Ok(map) = value.into_map() {
        let mut object = Map::new();
        for (key, cell) in map.iter() {
            object.insert(map_key(key), peek_to_json(cell)?);
        }
        return Ok(Json::Object(object));
    }
    let st = value.into_struct().map_err(reflect)?;
    let mut object = Map::new();
    for (field, cell) in st.fields() {
        object.insert(field.name.to_string(), peek_to_json(cell)?);
    }
    Ok(Json::Object(object))
}

fn write_into<'a>(partial: Partial<'a>, json: &Json, shape: &'static Shape) -> Result<Partial<'a>> {
    if is_option(shape) {
        return write_option(partial, json, shape);
    }
    if is_column(shape) {
        return set_value(partial, &cell_from_json(json, shape)?, shape, true);
    }
    match shape.def {
        Def::List(list) => write_list(partial, json, list.t),
        Def::Map(map) => write_map(partial, json, map.v()),
        _ => write_struct(partial, json, shape),
    }
}

fn write_option<'a>(partial: Partial<'a>, json: &Json, shape: &'static Shape) -> Result<Partial<'a>> {
    if matches!(json, Json::Null) {
        return partial.set_default().map_err(reflect);
    }
    let inner = option_inner(shape)?;
    let partial = write_into(partial.begin_some().map_err(reflect)?, json, inner)?;
    partial.end().map_err(reflect)
}

fn write_list<'a>(
    partial: Partial<'a>,
    json: &Json,
    child: &'static Shape,
) -> Result<Partial<'a>> {
    let items = json
        .as_array()
        .ok_or_else(|| DatasourceError::message("expected a json array"))?;
    let mut partial = partial.init_list_with_capacity(items.len()).map_err(reflect)?;
    for item in items {
        partial = write_into(partial.begin_list_item().map_err(reflect)?, item, child)?;
        partial = partial.end().map_err(reflect)?;
    }
    Ok(partial)
}

fn write_map<'a>(
    partial: Partial<'a>,
    json: &Json,
    value: &'static Shape,
) -> Result<Partial<'a>> {
    let object = json
        .as_object()
        .ok_or_else(|| DatasourceError::message("expected a json object"))?;
    let mut partial = partial.init_map().map_err(reflect)?;
    for (key, cell) in object {
        partial = partial.set_key(key.clone()).map_err(reflect)?;
        partial = write_into(partial.begin_value().map_err(reflect)?, cell, value)?;
        partial = partial.end().map_err(reflect)?;
    }
    Ok(partial)
}

fn write_struct<'a>(partial: Partial<'a>, json: &Json, shape: &'static Shape) -> Result<Partial<'a>> {
    let object = json
        .as_object()
        .ok_or_else(|| DatasourceError::message("expected a json object"))?;
    let mut partial = partial;
    for field in struct_fields(shape)? {
        partial = partial.begin_field(field.name).map_err(reflect)?;
        partial = match field_json(object, field.name) {
            Some(cell) => write_into(partial, cell, field.shape())?,
            None => partial.set_default().map_err(reflect)?,
        };
        partial = partial.end().map_err(reflect)?;
    }
    Ok(partial)
}

fn field_json<'a>(object: &'a Map<String, Json>, name: &str) -> Option<&'a Json> {
    object.get(name).or_else(|| object.get(&camel(name)))
}

fn camel(name: &str) -> String {
    let mut parts = name.split('_');
    let first = parts.next().unwrap_or_default();
    parts.fold(first.to_string(), |mut name, part| {
        let mut chars = part.chars();
        if let Some(head) = chars.next() {
            name.push(head.to_ascii_uppercase());
            name.extend(chars);
        }
        name
    })
}

fn map_key(value: Peek<'_, '_>) -> String {
    peek_value(value).map(|cell| cell.wire_text()).unwrap_or_default()
}

fn value_to_json(value: Value) -> Json {
    match value {
        Value::Null => Json::Null,
        Value::Bool(flag) => Json::Bool(flag),
        Value::Int(number) => Json::Number(Number::from(number)),
        Value::Float(number) => Number::from_f64(number).map(Json::Number).unwrap_or(Json::Null),
        Value::Text(text) => Json::String(text),
        Value::DateTime(date) => Json::String(Value::iso_datetime(date)),
        Value::Uuid(id) => Json::String(id.to_string()),
        Value::Decimal(number) => Json::String(number.to_string()),
        Value::Binary(bytes) => Json::String(Value::from(bytes).wire_text()),
    }
}

fn cell_from_json(json: &Json, shape: &'static Shape) -> Result<Value> {
    match json {
        Json::Null => Ok(Value::Null),
        Json::Bool(flag) => Ok(Value::Bool(*flag)),
        Json::Number(number) => number
            .as_i64()
            .map(Value::Int)
            .or_else(|| number.as_f64().map(Value::Float))
            .ok_or_else(|| DatasourceError::message("cannot read json number")),
        Json::String(text) => Ok(text_cell(text, shape)),
        Json::Array(_) | Json::Object(_) => Err(DatasourceError::message(format!(
            "cannot assign json to {}",
            shape.type_identifier
        ))),
    }
}

fn text_cell(text: &str, shape: &'static Shape) -> Value {
    if is_datetime(shape) {
        if let Ok(date) = DateTime::parse_from_rfc3339(text) {
            return Value::DateTime(date.with_timezone(&Utc));
        }
    }
    if shape.type_identifier.contains("Uuid") {
        if let Ok(id) = text.parse() {
            return Value::Uuid(id);
        }
    }
    if is_binary(shape) {
        return Value::from(base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            text,
        ).unwrap_or_default());
    }
    Value::from(text.to_string())
}

fn is_datetime(shape: &'static Shape) -> bool {
    shape.type_identifier.contains("DateTime")
}

fn is_binary(shape: &'static Shape) -> bool {
    matches!(shape.def, Def::List(list) if list.t.type_identifier == "u8")
}

fn is_option(shape: &'static Shape) -> bool {
    matches!(shape.def, Def::Option(_))
}

fn option_inner(shape: &'static Shape) -> Result<&'static Shape> {
    match shape.def {
        Def::Option(option) => Ok(option.t()),
        _ => Err(DatasourceError::message(format!("{} is not an option", shape.type_identifier))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use facet::Facet;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct Kid {
        id: i64,
        label: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct Pref {
        locale: String,
        timezone: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Facet)]
    struct House {
        id: i32,
        name: String,
        floor: Option<u32>,
        created: Option<DateTime<Utc>>,
        stamp: Option<Vec<u8>>,
        kids: Vec<Kid>,
        settings: HashMap<String, String>,
        prefs: HashMap<String, Pref>,
    }

    #[test]
    fn object_round_trips_nested_fields() {
        let house = House {
            id: 7,
            name: "ada".into(),
            floor: Some(2),
            created: Some(
                DateTime::parse_from_rfc3339("2024-01-02T03:04:05.000Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            stamp: Some(vec![1]),
            kids: vec![Kid {
                id: 2,
                label: "a".into(),
            }],
            settings: HashMap::from([("theme".into(), "dark".into())]),
            prefs: HashMap::from([(
                "home".into(),
                Pref {
                    locale: "en-GB".into(),
                    timezone: "Europe/London".into(),
                },
            )]),
        };
        let json = JsonObjectSerializer::serialize(&house).unwrap();
        assert_eq!(json["id"], 7);
        assert_eq!(json["name"], "ada");
        assert_eq!(json["floor"], 2);
        assert_eq!(json["kids"][0]["label"], "a");
        assert_eq!(json["settings"]["theme"], "dark");
        assert_eq!(json["prefs"]["home"]["locale"], "en-GB");
        assert_eq!(json["stamp"], "AQ==");
        let copied = JsonObjectSerializer::deserialize::<House>(&json).unwrap();
        assert_eq!(copied, house);
    }

    #[test]
    fn reads_camel_case_keys() {
        let house = JsonObjectSerializer::from_str::<House>(
            r#"{"id":3,"firstName":"skip","name":"ada","floor":0}"#,
        )
        .unwrap();
        assert_eq!(house.id, 3);
        assert_eq!(house.name, "ada");
        assert_eq!(house.floor, Some(0));
    }
}
