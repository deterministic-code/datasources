use crate::datasource_error::DatasourceError;

pub fn fail(provider: &str) -> DatasourceError {
    DatasourceError::not_implemented(provider)
}
