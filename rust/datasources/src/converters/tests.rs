use super::*;
use crate::record::Record;
use crate::value::Value;
use std::collections::HashMap;

struct User;

impl ConvertSchema for User {
    fn schema() -> Vec<(&'static str, &'static str)> {
        vec![
            ("id", "INTEGER"),
            ("username", "TEXT"),
            ("is_enabled", "BOOLEAN"),
            ("created", "DATETIME"),
        ]
    }
}

fn register_sqlite() {
    reset();
    use_provider(
        "sqlite",
        sql_provider_types(HashMap::new(), true, "integer"),
    );
}

fn rec(pairs: &[(&str, Value)]) -> Record {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

#[test]
fn infers_user_schema_and_round_trips() {
    register_sqlite();
    let mapping = mapping::<User>("sqlite", "rust").unwrap();
    assert_eq!(mapping.source, "sqlite");
    assert_eq!(mapping.destination, "rust");
    assert_eq!(mapping.schema.get("is_enabled").unwrap(), "BOOLEAN");
    let user = mapping
        .to(&rec(&[
            ("id", Value::Int(7)),
            ("username", Value::from("ada")),
            ("is_enabled", Value::Int(1)),
            ("created", Value::from("2020-01-02T03:04:05.000Z")),
        ]))
        .unwrap();
    assert_eq!(user.get("is_enabled"), Some(&Value::Bool(true)));
    let row = mapping.from(&user).unwrap();
    assert_eq!(row.get("is_enabled"), Some(&Value::Int(1)));
}

#[test]
fn merges_schema_and_converters() {
    register_sqlite();
    let twice = FieldConverter::new(
        |value| match value {
            Value::Int(n) => Ok(Value::Int(n * 2)),
            other => Ok(other.clone()),
        },
        |value| match value {
            Value::Int(n) => Ok(Value::Int(n / 2)),
            other => Ok(other.clone()),
        },
    );
    let mapping = mapping_with::<User>(
        "sqlite",
        "rust",
        HashMap::from([("username".into(), "VARCHAR(32)".into())]),
        HashMap::from([("id".into(), twice)]),
        HashMap::new(),
    )
    .unwrap();
    assert_eq!(mapping.schema.get("username").unwrap(), "VARCHAR(32)");
    let user = mapping.to(&rec(&[("id", Value::Int(3))])).unwrap();
    assert_eq!(user.get("id"), Some(&Value::Int(6)));
    assert_eq!(
        mapping.from(&rec(&[("id", Value::Int(6))])).unwrap().get("id"),
        Some(&Value::Int(3))
    );
}

#[test]
fn maps_without_schema() {
    register_sqlite();
    let mapping = mapping::<Cells>("sqlite", "rust").unwrap();
    assert!(mapping.schema.is_empty());
    let dest = mapping.to(&rec(&[("display_name", Value::from("a"))])).unwrap();
    assert_eq!(dest.get("display_name"), Some(&Value::from("a")));
}

#[test]
fn identity_provider_keeps_keys() {
    reset();
    use_provider("json", identity_provider_types());
    let mapping = mapping::<User>("json", "rust").unwrap();
    let dest = mapping.to(&rec(&[("is_enabled", Value::Bool(true))])).unwrap();
    assert_eq!(dest.get("is_enabled"), Some(&Value::Bool(true)));
}

#[test]
fn rejects_wrong_destination_and_missing_provider() {
    register_sqlite();
    assert!(mapping::<User>("sqlite", "csharp").is_err());
    reset();
    assert!(mapping::<User>("sqlite", "rust").is_err());
}

#[test]
fn builds_alias_tables() {
    let table = sql_provider_types(
        HashMap::from([
            ("VARCHAR".into(), "TEXT".into()),
            ("NOPE".into(), "MISSING".into()),
        ]),
        true,
        "integer",
    );
    assert!(table.rename);
    assert!(table.types.contains_key("VARCHAR"));
    assert!(table.types.contains_key("NOPE"));
    assert!(!sql_provider_types(HashMap::new(), false, "native").rename);
    assert_eq!(normalize_sql_type(" varchar(32) "), "VARCHAR");
    assert_eq!(normalize_sql_type("TINYINT(1)"), "TINYINT(1)");
    assert_eq!(normalize_sql_type("CHAR(36)"), "CHAR(36)");
    assert_eq!(normalize_sql_type("TEXT"), "TEXT");
    let native = canonical_converters("native");
    assert_eq!(
        native.get("BOOLEAN").unwrap().from(&Value::Bool(true)).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn field_converters_cover_branches() {
    let identity = identity_field_converter();
    assert_eq!(identity.to(&Value::from("a")).unwrap(), Value::from("a"));
    let integer = boolean_field_converter("integer");
    let native = boolean_field_converter("native");
    assert_eq!(integer.to(&Value::Int(1)).unwrap(), Value::Bool(true));
    assert_eq!(integer.to(&Value::Int(0)).unwrap(), Value::Bool(false));
    assert_eq!(integer.to(&Value::Bool(true)).unwrap(), Value::Bool(true));
    assert_eq!(integer.to(&Value::Null).unwrap(), Value::Null);
    assert_eq!(integer.from(&Value::Bool(true)).unwrap(), Value::Int(1));
    assert_eq!(integer.from(&Value::Bool(false)).unwrap(), Value::Int(0));
    assert_eq!(integer.from(&Value::Int(1)).unwrap(), Value::Int(1));
    assert_eq!(integer.from(&Value::Int(0)).unwrap(), Value::Int(0));
    assert_eq!(integer.from(&Value::Null).unwrap(), Value::Null);
    assert_eq!(native.from(&Value::Bool(false)).unwrap(), Value::Bool(false));
    assert_eq!(native.from(&Value::Int(1)).unwrap(), Value::Bool(true));
    assert_eq!(native.from(&Value::Int(0)).unwrap(), Value::Bool(false));
    assert!(integer.to(&Value::from("yes")).is_err());
    assert!(integer.from(&Value::from("yes")).is_err());
    let iso = "2020-01-02T03:04:05.000Z";
    let dt = date_time_field_converter();
    assert_eq!(dt.to(&Value::from(iso)).unwrap(), Value::from(iso));
    assert_eq!(dt.to(&Value::Null).unwrap(), Value::Null);
    assert_eq!(dt.from(&Value::from(iso)).unwrap(), Value::from(iso));
    assert_eq!(dt.from(&Value::Null).unwrap(), Value::Null);
    assert!(dt.to(&Value::from("nope")).is_err());
    assert!(dt.to(&Value::Int(1)).is_err());
    assert!(dt.from(&Value::Int(1)).is_err());
    let plain = decimal_field_converter(None);
    let sized = decimal_field_converter(Some((5, 2)));
    let whole = decimal_field_converter(Some((3, 0)));
    assert_eq!(plain.to(&Value::Null).unwrap(), Value::Null);
    assert_eq!(plain.to(&Value::Float(12.5)).unwrap(), Value::from("12.5"));
    assert_eq!(plain.to(&Value::Int(10)).unwrap(), Value::from("10"));
    assert_eq!(plain.from(&Value::from("1.20")).unwrap(), Value::from("1.20"));
    assert_eq!(sized.to(&Value::from("1.2")).unwrap(), Value::from("1.20"));
    assert_eq!(sized.to(&Value::Int(-3)).unwrap(), Value::from("-3.00"));
    assert_eq!(whole.to(&Value::from("12")).unwrap(), Value::from("12"));
    assert!(sized.to(&Value::from("1234.5")).is_err());
    assert!(plain.to(&Value::Bool(true)).is_err());
    assert!(sized.to(&Value::from("x")).is_err());
    assert!(plain.to(&Value::Float(f64::NAN)).is_err());
    let bin = binary_field_converter();
    assert_eq!(bin.to(&Value::from("aGk=")).unwrap(), Value::from("aGk="));
    assert_eq!(bin.to(&Value::Null).unwrap(), Value::Null);
    assert_eq!(bin.from(&Value::from("aGk=")).unwrap(), Value::from("aGk="));
    assert_eq!(bin.from(&Value::Null).unwrap(), Value::Null);
    assert!(bin.to(&Value::Int(1)).is_err());
    assert!(bin.from(&Value::Int(1)).is_err());
    let id = "550e8400-e29b-41d4-a716-446655440000";
    let uuid = uuid_field_converter();
    assert_eq!(uuid.to(&Value::from(id)).unwrap(), Value::from(id));
    assert_eq!(uuid.from(&Value::from(id)).unwrap(), Value::from(id));
    assert_eq!(uuid.to(&Value::Null).unwrap(), Value::Null);
    assert!(uuid.to(&Value::from("nope")).is_err());
    assert!(is_uuid(id));
    assert!(!is_uuid("550e8400e29b41d4a716446655440000"));
}

#[test]
fn unknown_sql_type_uses_identity() {
    register_sqlite();
    let mapping = mapping_with::<Cells>(
        "sqlite",
        "rust",
        HashMap::from([("label".into(), "FOO".into())]),
        HashMap::new(),
        HashMap::new(),
    )
    .unwrap();
    assert_eq!(
        mapping.to(&rec(&[("label", Value::from("x"))])).unwrap().get("label"),
        Some(&Value::from("x"))
    );
}
