use crate::MysqlDriver;
use datasources::Driver;
use datasources_tests::{assert_cells, assert_defaults, assert_types};

#[test]
fn type_converter() {
    let driver = MysqlDriver;
    assert_types(
        "mysql",
        driver.type_converter().expect("type map"),
        MysqlDriver::convert_type,
    );
}

#[test]
fn default_value_converter() {
    assert_defaults("mysql", MysqlDriver.default_value_converter());
}

#[test]
fn row_type_converters() {
    assert_cells("mysql", &MysqlDriver.row_type_converters());
}
