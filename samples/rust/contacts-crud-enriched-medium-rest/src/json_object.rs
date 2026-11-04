use chrono::{DateTime, Utc};
use datasources::facet::{Def, Facet, Field, HasFields, Partial, Peek, ScalarType, Shape, Type, UserType};
use datasources::{DatasourceError, Result, Value};
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
        return Ok(scalar_to_json(value));
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
        return write_column(partial, json, shape);
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

fn write_list<'a>(partial: Partial<'a>, json: &Json, child: &'static Shape) -> Result<Partial<'a>> {
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

fn write_map<'a>(partial: Partial<'a>, json: &Json, value: &'static Shape) -> Result<Partial<'a>> {
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
            None if is_datetime(field.shape()) => partial
                .set(DateTime::<Utc>::default())
                .map_err(reflect)?,
            None => partial.set_default().map_err(reflect)?,
        };
        partial = partial.end().map_err(reflect)?;
    }
    Ok(partial)
}

fn write_column<'a>(partial: Partial<'a>, json: &Json, shape: &'static Shape) -> Result<Partial<'a>> {
    if is_datetime(shape) {
        let date = match json {
            Json::String(text) => DateTime::parse_from_rfc3339(text)
                .map(|date| date.with_timezone(&Utc))
                .unwrap_or_default(),
            _ => DateTime::<Utc>::default(),
        };
        return partial.set(date).map_err(reflect);
    }
    if shape.type_identifier.contains("Uuid") {
        return match json {
            Json::String(text) => match text.parse::<datasources::uuid::Uuid>() {
                Ok(id) => partial.set(id).map_err(reflect),
                Err(_) => partial.set_default().map_err(reflect),
            },
            _ => partial.set_default().map_err(reflect),
        };
    }
    if is_binary(shape) {
        let bytes = match json {
            Json::String(text) => decode_base64(text),
            _ => Vec::new(),
        };
        return partial.set(bytes).map_err(reflect);
    }
    if shape.type_identifier.contains("Decimal") {
        if let Json::String(text) = json {
            if let Ok(number) = text.parse::<datasources::rust_decimal::Decimal>() {
                return partial.set(number).map_err(reflect);
            }
        }
    }
    match json {
        Json::Null => partial.set_default().map_err(reflect),
        Json::Bool(flag) => partial.set(*flag).map_err(reflect),
        Json::Number(number) => write_number(partial, number, shape),
        Json::String(text) => partial.set(text.clone()).map_err(reflect),
        Json::Array(_) | Json::Object(_) => Err(DatasourceError::message(format!(
            "cannot assign json to {}",
            shape.type_identifier
        ))),
    }
}

fn write_number<'a>(partial: Partial<'a>, number: &Number, shape: &'static Shape) -> Result<Partial<'a>> {
    match ScalarType::try_from_shape(shape) {
        Some(ScalarType::I32) => match number.as_i64() {
            Some(value) => partial.set(value as i32).map_err(reflect),
            None => partial.set_default().map_err(reflect),
        },
        Some(ScalarType::U32) => match number.as_u64() {
            Some(value) => partial.set(value as u32).map_err(reflect),
            None => partial.set_default().map_err(reflect),
        },
        Some(ScalarType::F64) => match number.as_f64() {
            Some(value) => partial.set(value).map_err(reflect),
            None => partial.set_default().map_err(reflect),
        },
        _ => match number.as_i64() {
            Some(value) => partial.set(value).map_err(reflect),
            None => match number.as_f64() {
                Some(value) => partial.set(value).map_err(reflect),
                None => Err(DatasourceError::message("cannot read json number")),
            },
        },
    }
}

fn scalar_to_json(value: Peek<'_, '_>) -> Json {
    value
        .get::<bool>()
        .ok()
        .map(|flag| Json::Bool(*flag))
        .or_else(|| value.get::<i32>().ok().map(|number| Json::Number(Number::from(*number))))
        .or_else(|| value.get::<u32>().ok().map(|number| Json::Number(Number::from(*number))))
        .or_else(|| value.get::<i64>().ok().map(|number| Json::Number(Number::from(*number))))
        .or_else(|| {
            value
                .get::<f64>()
                .ok()
                .and_then(|number| Number::from_f64(*number).map(Json::Number))
        })
        .or_else(|| value.get::<String>().ok().map(|text| Json::String(text.clone())))
        .or_else(|| {
            value
                .get::<DateTime<Utc>>()
                .ok()
                .map(|date| Json::String(Value::iso_datetime(*date)))
        })
        .or_else(|| {
            value
                .get::<datasources::uuid::Uuid>()
                .ok()
                .map(|id| Json::String(id.to_string()))
        })
        .or_else(|| {
            value
                .get::<datasources::rust_decimal::Decimal>()
                .ok()
                .map(|number| Json::String(number.to_string()))
        })
        .or_else(|| {
            value
                .get::<Vec<u8>>()
                .ok()
                .map(|bytes| Json::String(Value::from(bytes.clone()).wire_text()))
        })
        .unwrap_or(Json::Null)
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
    match scalar_to_json(value) {
        Json::String(text) => text,
        Json::Number(number) => number.to_string(),
        Json::Bool(flag) => flag.to_string(),
        _ => String::new(),
    }
}

fn decode_base64(text: &str) -> Vec<u8> {
    fn value(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }
    let bytes: Vec<u8> = text.bytes().filter_map(value).collect();
    bytes
        .chunks(4)
        .flat_map(|chunk| {
            let a = chunk.first().copied().unwrap_or(0);
            let b = chunk.get(1).copied().unwrap_or(0);
            let c = chunk.get(2).copied().unwrap_or(0);
            let d = chunk.get(3).copied().unwrap_or(0);
            [
                Some((a << 2) | (b >> 4)),
                (chunk.len() > 2).then_some(((b & 0x0f) << 4) | (c >> 2)),
                (chunk.len() > 3).then_some(((c & 0x03) << 6) | d),
            ]
        })
        .flatten()
        .collect()
}

fn struct_fields(shape: &'static Shape) -> Result<&'static [Field]> {
    match shape.ty {
        Type::User(UserType::Struct(st)) => Ok(st.fields),
        _ => Err(DatasourceError::message(format!("{} is not a struct", shape.type_identifier))),
    }
}

fn is_column(shape: &'static Shape) -> bool {
    let shape = if is_option(shape) {
        match option_inner(shape) {
            Ok(inner) => inner,
            Err(_) => return false,
        }
    } else {
        shape
    };
    match shape.def {
        Def::Map(_) => false,
        Def::List(list) => list.t.type_identifier == "u8",
        _ => !matches!(shape.ty, Type::User(UserType::Struct(_))),
    }
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

fn reflect(error: impl std::fmt::Display) -> DatasourceError {
    DatasourceError::message(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
    #[facet(crate = ::datasources::facet)]
    struct Kid {
        id: i64,
        label: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
    #[facet(crate = ::datasources::facet)]
    struct Pref {
        locale: String,
        timezone: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
    #[facet(crate = ::datasources::facet)]
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

    #[derive(Clone, Debug, Default, PartialEq, datasources::Facet)]
    #[facet(crate = ::datasources::facet)]
    struct Stamp {
        name: String,
        created: DateTime<Utc>,
    }

    #[test]
    fn omitted_datetime_defaults_to_epoch() {
        let stamp = JsonObjectSerializer::from_str::<Stamp>(r#"{"name":"ada"}"#).unwrap();
        assert_eq!(stamp.name, "ada");
        assert_eq!(stamp.created, DateTime::<Utc>::default());
    }
}
