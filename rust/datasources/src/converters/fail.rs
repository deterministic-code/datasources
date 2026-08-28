use crate::datasource_error::DatasourceError;
use crate::value::Value;

pub fn fail(value: &Value, target: &str) -> DatasourceError {
    DatasourceError::message(format!("Cannot convert {value:?} to {target}"))
}
