use datasources::{DatasourceError, Result};
use datasources_mysql::MysqlColumnType;
use datasources_postgres::PostgresColumnType;
use datasources_sqlite::SqliteColumnType;

pub fn map(provider: &str) -> Result<fn(&str) -> Result<String>> {
    match provider {
        "sqlite" => Ok(SqliteColumnType::map),
        "mysql" => Ok(MysqlColumnType::map),
        "postgres" => Ok(PostgresColumnType::map),
        _ => Err(DatasourceError::message(format!(
            "unknown provider {provider}"
        ))),
    }
}

pub fn map_type(provider: &str, canonical: &str) -> Result<String> {
    map(provider)?(canonical)
}
