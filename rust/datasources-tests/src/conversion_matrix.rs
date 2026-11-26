use chrono::{DateTime, Utc};
use datasources::{
    DefaultValueConverter, FieldSize, Mapping, Result, SpecField, TypeConverter, Value,
};
use rust_decimal::Decimal;
use serde_yaml::Value as Yaml;
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

pub fn assert_types(
    driver: &str,
    types: &dyn Mapping<String, String>,
    convert_field: impl Fn(&SpecField) -> Result<String>,
) {
    for row in rows(driver, "typeConverter") {
        let from = decode(&row["from"]);
        let reverse = flag(&row, "reverse");
        let got = if reverse {
            types.reverse_convert(&text(&from).to_string())
        } else {
            match from {
                Cell::Field(field) => convert_field(&field),
                other => types.convert(&text(&other).to_string()),
            }
        };
        expect(&row["to"], got);
    }
}

pub fn assert_defaults(driver: &str, defaults: &dyn DefaultValueConverter) {
    for row in rows(driver, "defaultValueConverter") {
        let spec = str_of(&row["type"]);
        let decoded = decode(&row["from"]);
        let from = text(&decoded).to_string();
        let got = if flag(&row, "reverse") {
            defaults.reverse_convert(spec, &from).map(Some)
        } else {
            defaults.convert(spec, &from)
        };
        expect_default(&row["to"], got);
    }
}

pub fn assert_cells(
    driver: &str,
    catalog: &HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>,
) {
    for row in rows(driver, "rowTypeConverters") {
        let spec = str_of(&row["type"]);
        let from = decode(&row["from"]);
        assert_type_name(str_of(&row["fromType"]), &from);
        let converter = catalog.get(spec).unwrap_or_else(|| panic!("{spec}"));
        let value = from.value();
        let got = if flag(&row, "reverse") {
            converter.reverse_convert(&value)
        } else {
            converter.convert(&value)
        };
        if let Some(error) = error_name(&row["to"]) {
            assert!(got.is_err(), "{spec} {error}");
            continue;
        }
        let expected = decode(&row["to"]);
        let actual = got.expect(spec);
        assert_type_name(str_of(&row["toType"]), &expected);
        assert_eq!(expected.value(), actual, "{spec}");
    }
}

enum Cell {
    Long(i64),
    Text(String),
    Bool(bool),
    Float(f64),
    Decimal(Decimal),
    DateTime(DateTime<Utc>),
    Uuid(Uuid),
    Bytes(Vec<u8>),
    Field(SpecField),
}

impl Cell {
    fn value(&self) -> Value {
        match self {
            Cell::Long(n) => Value::Int(*n),
            Cell::Text(text) => Value::from(text.as_str()),
            Cell::Bool(flag) => Value::Bool(*flag),
            Cell::Float(n) => Value::Float(*n),
            Cell::Decimal(n) => Value::Decimal(*n),
            Cell::DateTime(date) => Value::DateTime(*date),
            Cell::Uuid(id) => Value::Uuid(*id),
            Cell::Bytes(bytes) => Value::Binary(bytes.clone()),
            Cell::Field(_) => panic!("not a cell value"),
        }
    }
}

fn expect(expected: &Yaml, got: Result<String>) {
    if let Some(error) = error_name(expected) {
        assert!(got.is_err(), "{error}");
        return;
    }
    assert_eq!(str_of(expected), got.expect("type"));
}

fn expect_default(expected: &Yaml, got: Result<Option<String>>) {
    if let Some(error) = error_name(expected) {
        assert!(got.is_err(), "{error}");
        return;
    }
    if expected.is_null() {
        assert_eq!(got.expect("default"), None);
        return;
    }
    assert_eq!(got.expect("default").as_deref(), Some(str_of(expected)));
}

fn decode(node: &Yaml) -> Cell {
    let Yaml::Mapping(map) = node else {
        return Cell::Text(str_of(node).into());
    };
    if let Some(field) = map.get(Yaml::String("field".into())) {
        return Cell::Field(spec(field));
    }
    let (key, value) = map.iter().next().expect("tag");
    match str_of(key) {
        "long" => Cell::Long(i64_of(value)),
        "text" => Cell::Text(str_of(value).into()),
        "bool" => Cell::Bool(bool_of(value)),
        "float" => Cell::Float(f64_of(value)),
        "decimal" => Cell::Decimal(str_of(value).parse().expect("decimal")),
        "datetime" => Cell::DateTime(
            DateTime::parse_from_rfc3339(str_of(value))
                .expect("datetime")
                .with_timezone(&Utc),
        ),
        "guid" => Cell::Uuid(Uuid::parse_str(str_of(value)).expect("guid")),
        "bytes" => Cell::Bytes(bytes_of(value)),
        "base64" => Cell::Text(str_of(value).into()),
        "latin1" => Cell::Text(bytes_of(value).into_iter().map(char::from).collect()),
        other => panic!("{other}"),
    }
}

fn spec(node: &Yaml) -> SpecField {
    let Yaml::Mapping(map) = node else {
        panic!("field");
    };
    SpecField {
        type_name: str_of(&map[&Yaml::String("type".into())]).into(),
        name: map
            .get(Yaml::String("name".into()))
            .map(str_of)
            .map(str::to_string),
        size: map.get(Yaml::String("size".into())).and_then(size_of),
        default_value: None,
    }
}

fn size_of(node: &Yaml) -> Option<FieldSize> {
    match node {
        Yaml::String(text) if text == "unlimited" => Some(FieldSize::Unlimited),
        Yaml::Number(n) => Some(FieldSize::Length(n.as_i64()? as i32)),
        Yaml::Sequence(items) if items.len() == 2 => Some(FieldSize::Decimal(
            items[0].as_i64()? as i32,
            items[1].as_i64()? as i32,
        )),
        _ => None,
    }
}

fn assert_type_name(expected: &str, cell: &Cell) {
    let actual = match cell {
        Cell::Long(_) => "i64",
        Cell::Text(_) => "String",
        Cell::Bool(_) => "bool",
        Cell::Float(_) => "f64",
        Cell::Decimal(_) => "Decimal",
        Cell::DateTime(_) => "DateTime<Utc>",
        Cell::Uuid(_) => "Uuid",
        Cell::Bytes(_) => "Vec<u8>",
        Cell::Field(_) => "",
    };
    assert_eq!(expected, actual);
}

fn error_name(node: &Yaml) -> Option<&str> {
    node.as_mapping()
        .and_then(|map| map.get(Yaml::String("error".into())))
        .map(str_of)
}

fn rows(driver: &str, table: &str) -> Vec<Yaml> {
    let text = fs::read_to_string(path()).expect("DriverConversions.yaml");
    let root: Yaml = serde_yaml::from_str(&text).expect("yaml");
    root[driver]["rust"][table]
        .as_sequence()
        .expect(table)
        .clone()
}

fn path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo")
        .join("docs/DriverConversions.yaml")
}

fn flag(row: &Yaml, name: &str) -> bool {
    row.as_mapping()
        .and_then(|map| map.get(Yaml::String(name.into())))
        .and_then(Yaml::as_bool)
        .unwrap_or(false)
}

fn text(cell: &Cell) -> &str {
    match cell {
        Cell::Text(text) => text,
        _ => panic!("text"),
    }
}

fn str_of(node: &Yaml) -> &str {
    node.as_str().unwrap_or_else(|| panic!("{node:?}"))
}

fn i64_of(node: &Yaml) -> i64 {
    node.as_i64().unwrap_or_else(|| panic!("{node:?}"))
}

fn f64_of(node: &Yaml) -> f64 {
    node.as_f64()
        .or_else(|| node.as_i64().map(|n| n as f64))
        .unwrap_or_else(|| panic!("{node:?}"))
}

fn bool_of(node: &Yaml) -> bool {
    node.as_bool().unwrap_or_else(|| panic!("{node:?}"))
}

fn bytes_of(node: &Yaml) -> Vec<u8> {
    node.as_sequence()
        .expect("bytes")
        .iter()
        .map(|item| item.as_u64().expect("byte") as u8)
        .collect()
}
