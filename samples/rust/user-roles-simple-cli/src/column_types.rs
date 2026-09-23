use datasources::{DatasourceError, Result};

pub fn map(provider: &str) -> Result<fn(&str) -> Result<String>> {
    match provider {
        "sqlite" => Ok(sqlite),
        "mysql" => Ok(mysql),
        "postgres" => Ok(postgres),
        _ => Err(DatasourceError::message(format!(
            "unknown provider {provider}"
        ))),
    }
}

pub fn map_type(provider: &str, canonical: &str) -> Result<String> {
    map(provider)?(canonical)
}

fn sqlite(canonical: &str) -> Result<String> {
    Ok(match canonical {
        "int" => "INTEGER",
        "uuid" | "string" | "datetime" => "TEXT",
        "bool" => "INTEGER",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "binary" => "BLOB",
        _ => {
            return Err(DatasourceError::message(format!(
                "Unknown sqlite type \"{canonical}\""
            )))
        }
    }
    .into())
}

fn mysql(canonical: &str) -> Result<String> {
    Ok(match canonical {
        "int" => "INT",
        "uuid" => "CHAR(36)",
        "string" | "datetime" => "TEXT",
        "bool" => "TINYINT",
        "decimal" => "DECIMAL(18,4)",
        "float" => "DOUBLE",
        "binary" => "BLOB",
        _ => {
            return Err(DatasourceError::message(format!(
                "Unknown mysql type \"{canonical}\""
            )))
        }
    }
    .into())
}

fn postgres(canonical: &str) -> Result<String> {
    Ok(match canonical {
        "int" => "INTEGER",
        "uuid" => "UUID",
        "string" => "TEXT",
        "bool" => "BOOLEAN",
        "decimal" => "NUMERIC",
        "float" => "DOUBLE PRECISION",
        "datetime" => "TIMESTAMPTZ",
        "binary" => "BYTEA",
        _ => {
            return Err(DatasourceError::message(format!(
                "Unknown postgres type \"{canonical}\""
            )))
        }
    }
    .into())
}
