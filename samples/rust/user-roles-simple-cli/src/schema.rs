use datasources::{CreateTableStatement, DatasourceError, Driver, Result, Value};

pub fn flag(provider: &str, value: bool) -> Value {
    if provider == "sqlite" {
        Value::Int(if value { 1 } else { 0 })
    } else {
        Value::Bool(value)
    }
}

pub fn ident(provider: &str, name: &str) -> String {
    quote(provider, name)
}

pub fn qualified(provider: &str, name: &str) -> String {
    name.split('.')
        .map(|part| quote(provider, part))
        .collect::<Vec<_>>()
        .join(".")
}

fn quote(provider: &str, id: &str) -> String {
    match provider {
        "mysql" => format!("`{}`", id.replace('`', "``")),
        _ => format!("\"{}\"", id.replace('"', "\"\"")),
    }
}

pub fn tables(provider: &str, driver: &dyn Driver) -> Result<Vec<CreateTableStatement>> {
    if !matches!(provider, "sqlite" | "mysql" | "postgres") {
        return Err(DatasourceError::message(format!("unknown provider {provider}")));
    }
    Ok(datasources_sql_specs::schema_tables(
        "user-roles-cli",
        provider,
        |id| ident(provider, id),
        |canonical| {
            if provider == "sqlite" {
                driver.column_type(canonical).expect("column type")
            } else {
                canonical.to_string()
            }
        },
        None,
        false,
    ))
}

pub fn mysql_procedures() -> Vec<&'static str> {
    include_str!("../../../user-roles-simple-cli-shared/mysql-procedures.sql")
        .split("---")
        .map(str::trim)
        .filter(|sql| !sql.is_empty())
        .collect()
}
