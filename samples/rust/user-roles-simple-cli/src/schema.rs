use datasources::{CreateTableStatement, Driver, Result, Value};

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

pub fn tables(provider: &str, _driver: &dyn Driver) -> Result<Vec<CreateTableStatement>> {
    Ok(datasources_sql_specs::schema_tables(
        "user-roles-cli",
        |id| ident(provider, id),
        None,
    ))
}

pub fn mysql_procedures() -> Vec<&'static str> {
    include_str!("../../../user-roles-simple-cli-shared/mysql-procedures.sql")
        .split("---")
        .map(str::trim)
        .filter(|sql| !sql.is_empty())
        .collect()
}
