use crate::PostgresDriver;
use datasources::Driver;
use datasources_tests::{assert_cells, assert_defaults, assert_types};

#[test]
fn type_converter() {
    let driver = PostgresDriver;
    assert_types(
        "postgres",
        driver.type_converter().expect("type map"),
        PostgresDriver::convert_type,
    );
}

#[test]
fn default_value_converter() {
    assert_defaults("postgres", PostgresDriver.default_value_converter());
}

#[test]
fn row_type_converters() {
    assert_cells("postgres", &PostgresDriver.row_type_converters());
}
