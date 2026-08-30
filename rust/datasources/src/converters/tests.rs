use crate::converters::{RowConverter, TypeConverter};
use crate::row::Row;
use crate::value::Value;

fn record(pairs: &[(&str, Value)]) -> Row {
    pairs
        .iter()
        .map(|(k, v)| ((*k).to_string(), v.clone()))
        .collect()
}

#[test]
fn type_converter_round_trips_rows() {
    let converter = TypeConverter::new("users", vec!["id".into(), "name".into()]);
    let row = record(&[("id", Value::Int(7)), ("name", Value::from("ada"))]);
    let dest = converter.convert(&row).unwrap();
    assert_eq!(dest, row);
    assert_eq!(converter.reverse_convert(&dest).unwrap(), row);
    assert_eq!(converter.field_names(), vec!["id", "name"]);
}

#[test]
fn row_converter_clones() {
    let rows = RowConverter;
    let row = record(&[("display_name", Value::from("a"))]);
    assert_eq!(rows.convert(&row).unwrap(), row);
    assert_eq!(rows.reverse_convert(&row).unwrap(), row);
}
