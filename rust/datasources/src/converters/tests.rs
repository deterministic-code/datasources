use crate::converters::{RowConverter, TypeConverter, ValueConverter};
use crate::row::Row;
use crate::value::Value;
use std::collections::HashMap;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

#[test]
fn type_converter_round_trips_rows() {
    let converter = TypeConverter::new(vec!["id".into(), "name".into()]);
    let row = record(&[("id", Value::Int(7)), ("name", Value::from("ada"))]);
    let dest = converter.convert(&row).unwrap();
    assert_eq!(dest, row);
    assert_eq!(converter.reverse_convert(&dest).unwrap(), row);
    assert_eq!(converter.fields, vec!["id", "name"]);
}

#[test]
fn row_converter_clones() {
    let rows = RowConverter::default();
    let row = record(&[("display_name", Value::from("a"))]);
    assert_eq!(rows.convert(&row).unwrap(), row);
    assert_eq!(rows.reverse_convert(&row).unwrap(), row);
}

#[test]
fn field_override_beats_type_default() {
    let only_one = ValueConverter::new(
        |value| {
            Ok(match value {
                Value::Int(number) => Value::Bool(*number == 1),
                other => other.clone(),
            })
        },
        |value| Ok(value.clone()),
    );
    let schema = HashMap::from([("is_enabled".into(), "bool".into())]);
    let converter = TypeConverter::new(vec!["is_enabled".into()])
        .with_field_types(schema.clone())
        .with_fields(HashMap::from([("is_enabled".into(), only_one)]));
    let by_type = TypeConverter::new(vec!["is_enabled".into()]).with_field_types(schema.clone());
    let row = record(&[("is_enabled", Value::Int(2))]);
    assert_eq!(by_type.convert(&row).unwrap()["is_enabled"], Value::Bool(true));
    assert_eq!(
        converter.convert(&row).unwrap()["is_enabled"],
        Value::Bool(false)
    );
    let typed = TypeConverter::new(vec!["is_enabled".into()]).with_field_types(schema);
    assert_eq!(
        typed.convert(&record(&[("is_enabled", Value::from("true"))])).unwrap()["is_enabled"],
        Value::Bool(true)
    );
    assert_eq!(
        typed.convert(&record(&[("is_enabled", Value::Null)])).unwrap()["is_enabled"],
        Value::Null
    );
    assert_eq!(
        typed.convert(&record(&[("is_enabled", Value::Float(1.5))])).unwrap()["is_enabled"],
        Value::Float(1.5)
    );
}
