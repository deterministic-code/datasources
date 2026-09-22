use datasources::{DatasourceError, Result};

pub fn hash(password: &str) -> Result<String> {
    bcrypt::hash(password, 4).map_err(|error| DatasourceError::message(error.to_string()))
}
