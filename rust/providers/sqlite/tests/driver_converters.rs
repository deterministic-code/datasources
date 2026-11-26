use crate::SqliteDriver;
use datasources::Driver;
use datasources_tests::{assert_cells, assert_defaults, assert_types};

#[test]
fn type_converter() {
    let driver = SqliteDriver;
    assert_types(
        "sqlite",
        driver.type_converter().expect("type map"),
        SqliteDriver::convert_type,
    );
}

#[test]
fn default_value_converter() {
    let defaults = SqliteDriver.default_value_converter();
    assert_defaults("sqlite", defaults);
    assert!(defaults
        .convert("uuid", "NewId")
        .unwrap()
        .unwrap()
        .contains("randomblob"));
}

#[test]
fn row_type_converters() {
    assert_cells("sqlite", &SqliteDriver.row_type_converters());
}
