use datasources::{
    Column, ColumnDefault, CreateTableStatement, DatasourceError, Driver, ForeignKey, NotNull,
    PrimaryKey, Result, TableConstraint, Value,
};

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

fn stored(provider: &str, driver: &dyn Driver, canonical: &str) -> Result<String> {
    match provider {
        "sqlite" => driver.column_type(canonical),
        _ => Ok(canonical.to_string()),
    }
}

fn id_column(provider: &str, driver: &dyn Driver) -> Result<Column> {
    match provider {
        "sqlite" => Ok(Column::new(
            "id",
            format!("{} PRIMARY KEY", driver.column_type("int")?),
        )),
        "mysql" | "postgres" => Ok(Column::new("id", "int").identity()),
        _ => Err(DatasourceError::message(format!(
            "unknown provider {provider}"
        ))),
    }
}

fn id_key(provider: &str, name: &str) -> Vec<TableConstraint> {
    match provider {
        "sqlite" => vec![],
        _ => vec![PrimaryKey::new(name, &["id"])],
    }
}

fn required() -> NotNull {
    NotNull::new(None)
}

pub fn tables(provider: &str, driver: &dyn Driver) -> Result<Vec<CreateTableStatement>> {
    let text = stored(provider, driver, "string")?;
    let flag = stored(provider, driver, "bool")?;
    let created = stored(provider, driver, "datetime")?;
    let updated = stored(provider, driver, "binary")?;
    let number = stored(provider, driver, "int")?;
    let enabled = ColumnDefault::new("1", None);
    let users = ident(provider, "users");
    let roles = ident(provider, "roles");
    let groups = ident(provider, "groups");
    let user_groups = ident(provider, "user_groups");
    let user_roles = ident(provider, "user_roles");
    Ok(vec![
        CreateTableStatement::new(
            &users,
            &[
                id_column(provider, driver)?,
                Column::new("username", &text).not_null(required()),
                Column::new("password", &text).not_null(required()),
                Column::new("is_enabled", &flag)
                    .default(enabled.clone())
                    .not_null(required()),
                Column::new("created", &created).not_null(required()),
                Column::new("updated", &updated).not_null(required()),
            ],
            &id_key(provider, "pk_users"),
        ),
        CreateTableStatement::new(
            &roles,
            &[
                id_column(provider, driver)?,
                Column::new("name", &text).not_null(required()),
                Column::new("is_enabled", &flag)
                    .default(enabled.clone())
                    .not_null(required()),
                Column::new("created", &created).not_null(required()),
                Column::new("updated", &updated).not_null(required()),
            ],
            &id_key(provider, "pk_roles"),
        ),
        CreateTableStatement::new(
            &groups,
            &[
                id_column(provider, driver)?,
                Column::new("ugly_name", &text).not_null(required()),
                Column::new("is_enabled", &flag)
                    .default(enabled.clone())
                    .not_null(required()),
                Column::new("created", &created).not_null(required()),
                Column::new("updated", &updated).not_null(required()),
            ],
            &id_key(provider, "pk_groups"),
        ),
        CreateTableStatement::new(
            &user_groups,
            &[
                id_column(provider, driver)?,
                Column::new("user_id", &number).not_null(required()),
                Column::new("group_id", &number).not_null(required()),
                Column::new("created", &created).not_null(required()),
                Column::new("updated", &updated).not_null(required()),
            ],
            &[
                id_key(provider, "pk_user_groups"),
                vec![
                    ForeignKey::new(
                        "fk_user_groups_user",
                        &["user_id"],
                        &users,
                        &["id"],
                        None,
                        None,
                    ),
                    ForeignKey::new(
                        "fk_user_groups_group",
                        &["group_id"],
                        &groups,
                        &["id"],
                        None,
                        None,
                    ),
                ],
            ]
            .concat(),
        ),
        CreateTableStatement::new(
            &user_roles,
            &[
                id_column(provider, driver)?,
                Column::new("user_id", &number).not_null(required()),
                Column::new("bad_role_id", &number).not_null(required()),
                Column::new("created", &created).not_null(required()),
                Column::new("updated", &updated).not_null(required()),
            ],
            &[
                id_key(provider, "pk_user_roles"),
                vec![
                    ForeignKey::new(
                        "fk_user_roles_user",
                        &["user_id"],
                        &users,
                        &["id"],
                        None,
                        None,
                    ),
                    ForeignKey::new(
                        "fk_user_roles_role",
                        &["bad_role_id"],
                        &roles,
                        &["id"],
                        None,
                        None,
                    ),
                ],
            ]
            .concat(),
        ),
    ])
}

pub fn mysql_procedures() -> Vec<&'static str> {
    include_str!("../../../user-roles-simple-cli-shared/mysql-procedures.sql")
        .split("---")
        .map(str::trim)
        .filter(|sql| !sql.is_empty())
        .collect()
}
