use crate::datasource_error::DatasourceError;

pub type Result<T> = std::result::Result<T, DatasourceError>;
