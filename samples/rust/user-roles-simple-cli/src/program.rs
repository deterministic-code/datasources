use chrono::Utc;
use datasources::{
    Command, DatasourceError, DefaultStatementTextRenderer, ExecuteStoredProcedureStatement,
    FlatToNestedMapper, IWhere, InsertStatement, Join, MappedRow, MappedValue, Mapper, Operator,
    OrderBy, Parameter, Result, SelectStatement, SelectStatementOptions, TransformMapper,
    TypeMap, TypeMapField, TypeMapGroup, UpdateStatement, Value, Where,
};
use futures_util::TryStreamExt;
use serde::Serialize;

pub const USAGE: &str = "\
add-user <username> <password>
add-role <name>
add-group <name>
add-user-role <username> <role>
add-user-group <username> <group>
enable-user <username>
disable-user <username>
list-users-roles-groups";

pub fn names() -> TransformMapper<String, String> {
    TransformMapper::new(
        |name: String| match name.as_str() {
            "name" => "ugly_name".into(),
            "role_id" => "bad_role_id".into(),
            other => other.into(),
        },
        |name: String| match name.as_str() {
            "ugly_name" => "name".into(),
            "bad_role_id" => "role_id".into(),
            other => other.into(),
        },
    )
}

pub fn map_name(name: &str) -> String {
    Mapper::map(&names(), name.to_string())
}

pub fn reverse_map_name(name: &str) -> String {
    Mapper::reverse_map(&names(), name.to_string())
}

pub fn list_query() -> SelectStatement {
    let fields = [
        "users.id AS id",
        "users.username AS username",
        "users.is_enabled AS is_enabled",
        "users.created AS created",
        "roles.id AS role_id",
        "roles.name AS role_name",
        "roles.is_enabled AS role_is_enabled",
        "groups.id AS group_id",
        "groups.ugly_name AS group_name",
        "groups.is_enabled AS group_is_enabled",
    ];
    let order = [
        OrderBy::asc("users.id"),
        OrderBy::asc("roles.id"),
        OrderBy::asc("groups.id"),
    ];
    SelectStatement::new(SelectStatementOptions {
        joins: &[
            Join::left("user_roles", "users.id", "user_roles.user_id"),
            Join::left("roles", "user_roles.bad_role_id", "roles.id"),
            Join::left("user_groups", "users.id", "user_groups.user_id"),
            Join::left("groups", "user_groups.group_id", "groups.id"),
        ],
        order_by: &order,
        ..SelectStatementOptions::new("users", &fields)
    })
}

pub fn list_query_text() -> String {
    DefaultStatementTextRenderer::instance().render(&list_query())
}

pub fn hash(password: &str) -> Result<String> {
    bcrypt::hash(password, 4).map_err(|error| DatasourceError::message(error.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parsed {
    pub verb: String,
    pub values: Vec<String>,
}

pub fn parse(args: &[String]) -> Result<Parsed> {
    match args {
        [cmd, username, password] if cmd == "add-user" => Ok(Parsed {
            verb: cmd.clone(),
            values: vec![username.clone(), password.clone()],
        }),
        [cmd, name] if cmd == "add-role" || cmd == "add-group" => Ok(Parsed {
            verb: cmd.clone(),
            values: vec![name.clone()],
        }),
        [cmd, username, right] if cmd == "add-user-role" || cmd == "add-user-group" => Ok(Parsed {
            verb: cmd.clone(),
            values: vec![username.clone(), right.clone()],
        }),
        [cmd, username] if cmd == "enable-user" || cmd == "disable-user" => Ok(Parsed {
            verb: cmd.clone(),
            values: vec![username.clone()],
        }),
        [cmd] if cmd == "list-users-roles-groups" => Ok(Parsed {
            verb: cmd.clone(),
            values: vec![],
        }),
        [cmd, ..] if cmd == "add-user" => Err(DatasourceError::message(
            "Usage: add-user <username> <password>",
        )),
        [cmd, ..] if cmd == "add-role" => Err(DatasourceError::message("Usage: add-role <name>")),
        [cmd, ..] if cmd == "add-group" => Err(DatasourceError::message("Usage: add-group <name>")),
        [cmd, ..] if cmd == "add-user-role" => Err(DatasourceError::message(
            "Usage: add-user-role <username> <role>",
        )),
        [cmd, ..] if cmd == "add-user-group" => Err(DatasourceError::message(
            "Usage: add-user-group <username> <group>",
        )),
        [cmd, ..] if cmd == "enable-user" => {
            Err(DatasourceError::message("Usage: enable-user <username>"))
        }
        [cmd, ..] if cmd == "disable-user" => {
            Err(DatasourceError::message("Usage: disable-user <username>"))
        }
        [cmd, ..] => Err(DatasourceError::message(format!(
            "Unknown command: {cmd}\n{USAGE}"
        ))),
        _ => Err(DatasourceError::message(USAGE)),
    }
}

pub async fn run(
    command: &dyn Command,
    args: &[String],
    hash_password: impl Fn(&str) -> Result<String>,
    procedures: bool,
) -> Result<String> {
    let parsed = parse(args)?;
    let provider = command.connection().provider().to_string();
    match parsed.verb.as_str() {
        "add-user" => {
            write(
                command,
                &provider,
                "users",
                "add_user",
                &[
                    Parameter::new("username", parsed.values[0].as_str()),
                    Parameter::new("password", hash_password(&parsed.values[1])?),
                    Parameter::new("is_enabled", true),
                    created(),
                ],
                &[
                    Parameter::new("p_username", parsed.values[0].as_str()),
                    Parameter::new("p_password", hash_password(&parsed.values[1])?),
                    Parameter::new("p_is_enabled", true),
                    Parameter::new("p_created", created().value.clone()),
                    updated("p_updated"),
                ],
                procedures,
            )
            .await
        }
        "add-role" => {
            write(
                command,
                &provider,
                "roles",
                "add_role",
                &[
                    Parameter::new("name", parsed.values[0].as_str()),
                    Parameter::new("is_enabled", true),
                    created(),
                ],
                &[
                    Parameter::new("p_name", parsed.values[0].as_str()),
                    Parameter::new("p_is_enabled", true),
                    Parameter::new("p_created", created().value.clone()),
                    updated("p_updated"),
                ],
                procedures,
            )
            .await
        }
        "add-group" => {
            write(
                command,
                &provider,
                "groups",
                "add_group",
                &[
                    Parameter::new(map_name("name"), parsed.values[0].as_str()),
                    Parameter::new("is_enabled", true),
                    created(),
                ],
                &[
                    Parameter::new("p_name", parsed.values[0].as_str()),
                    Parameter::new("p_is_enabled", true),
                    Parameter::new("p_created", created().value.clone()),
                    updated("p_updated"),
                ],
                procedures,
            )
            .await
        }
        "add-user-role" => {
            link(
                command,
                &provider,
                &parsed.values[0],
                "roles",
                "name",
                &parsed.values[1],
                "user_roles",
                &map_name("role_id"),
                "add_user_role",
                procedures,
            )
            .await
        }
        "add-user-group" => {
            link(
                command,
                &provider,
                &parsed.values[0],
                "groups",
                &map_name("name"),
                &parsed.values[1],
                "user_groups",
                "group_id",
                "add_user_group",
                procedures,
            )
            .await
        }
        "enable-user" => set_enabled(command, &provider, &parsed.values[0], true, procedures).await,
        "disable-user" => set_enabled(command, &provider, &parsed.values[0], false, procedures).await,
        "list-users-roles-groups" => list(command).await,
        _ => Err(DatasourceError::message("empty command")),
    }
}

async fn write(
    command: &dyn Command,
    provider: &str,
    table: &str,
    procedure: &str,
    insert: &[Parameter],
    call: &[Parameter],
    procedures: bool,
) -> Result<String> {
    if procedures && provider == "mysql" {
        command
            .execute_statement(
                &ExecuteStoredProcedureStatement::new(procedure, call),
                Some(call),
            )
            .await?;
    } else {
        command
            .execute_statement(&InsertStatement::new(table, insert), Some(insert))
            .await?;
    }
    Ok(String::new())
}

async fn link(
    command: &dyn Command,
    provider: &str,
    username: &str,
    right_table: &str,
    right_column: &str,
    right_value: &str,
    link_table: &str,
    link_column: &str,
    procedure: &str,
    procedures: bool,
) -> Result<String> {
    let user_id = lookup_id(command, "users", "username", username).await?;
    let right_id = lookup_id(command, right_table, right_column, right_value).await?;
    write(
        command,
        provider,
        link_table,
        procedure,
        &[
            Parameter::new("user_id", user_id),
            Parameter::new(link_column, right_id),
            created(),
        ],
        &[
            Parameter::new("p_user_id", user_id),
            Parameter::new(
                if link_column == "bad_role_id" {
                    "p_role_id"
                } else {
                    "p_group_id"
                },
                right_id,
            ),
            Parameter::new("p_created", created().value.clone()),
            updated("p_updated"),
        ],
        procedures,
    )
    .await
}

async fn set_enabled(
    command: &dyn Command,
    provider: &str,
    username: &str,
    enabled: bool,
    procedures: bool,
) -> Result<String> {
    if procedures && provider == "mysql" {
        let fields = [
            Parameter::new("p_username", username),
            Parameter::new("p_is_enabled", enabled),
            updated("p_updated"),
        ];
        command
            .execute_statement(
                &ExecuteStoredProcedureStatement::new("set_user_enabled", &fields),
                Some(&fields),
            )
            .await?;
        return Ok(String::new());
    }
    let id = lookup_id(command, "users", "username", username).await?;
    let fields = [Parameter::new("is_enabled", enabled)];
    let where_clause = Where::new(Operator::equals("id", id));
    let mut parameters = fields.to_vec();
    parameters.extend(where_clause.parameters());
    command
        .execute_statement(
            &UpdateStatement::new("users", &fields, Some(&where_clause)),
            Some(&parameters),
        )
        .await?;
    Ok(String::new())
}

async fn lookup_id(command: &dyn Command, table: &str, column: &str, value: &str) -> Result<i64> {
    let where_clause = Where::new(Operator::equals(column, value));
    let query = SelectStatement::new(SelectStatementOptions {
        where_clause: Some(&where_clause),
        ..SelectStatementOptions::new(table, &["id"])
    });
    let parameters = where_clause.parameters();
    let reader = command
        .execute_reader_statement(&query, Some(&parameters))
        .await?;
    if !reader.next().await? {
        return Err(DatasourceError::message(format!(
            "{table} '{value}' not found"
        )));
    }
    match reader.current().and_then(|row| row.get("id").cloned()) {
        Some(Value::Int(number)) => Ok(number),
        _ => Err(DatasourceError::message(format!(
            "{table} '{value}' not found"
        ))),
    }
}

async fn list(command: &dyn Command) -> Result<String> {
    let reader = command
        .execute_reader_statement(&list_query(), None)
        .await?;
    let rows = reader.read().try_collect().await?;
    let mapped = Mapper::map(&FlatToNestedMapper::new(users_view()), rows);
    serde_json::to_string(&Mapper::map(&UserViewMapper, mapped))
        .map_err(|error| DatasourceError::message(error.to_string()))
}

fn users_view() -> TypeMap {
    TypeMap::new(
        "id",
        vec![
            ("id".into(), TypeMapField::column("id")),
            ("username".into(), TypeMapField::column("username")),
            ("is_enabled".into(), TypeMapField::column("is_enabled")),
            ("created".into(), TypeMapField::column("created")),
            (
                "roles".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "role_id",
                    vec![
                        ("id".into(), TypeMapField::column("role_id")),
                        ("name".into(), TypeMapField::column("role_name")),
                        ("is_enabled".into(), TypeMapField::column("role_is_enabled")),
                    ],
                )),
            ),
            (
                "groups".into(),
                TypeMapField::Group(TypeMapGroup::new(
                    "group_id",
                    vec![
                        ("id".into(), TypeMapField::column("group_id")),
                        ("name".into(), TypeMapField::column("group_name")),
                        (
                            "is_enabled".into(),
                            TypeMapField::column("group_is_enabled"),
                        ),
                    ],
                )),
            ),
        ],
    )
}

#[derive(Serialize)]
struct UserView {
    id: i64,
    username: String,
    is_enabled: bool,
    created: String,
    roles: Vec<ChildView>,
    groups: Vec<ChildView>,
}

#[derive(Serialize)]
struct ChildView {
    id: i64,
    name: String,
    is_enabled: bool,
}

struct UserViewMapper;

impl Mapper<Vec<MappedRow>, Vec<UserView>> for UserViewMapper {
    fn map(&self, rows: Vec<MappedRow>) -> Vec<UserView> {
        let mut users: Vec<UserView> = rows
            .iter()
            .map(|row| {
                let mut roles = Self::children(row, "roles");
                roles.sort_by_key(|role| role.id);
                let mut groups = Self::children(row, "groups");
                groups.sort_by_key(|group| group.id);
                UserView {
                    id: Self::id(row, "id"),
                    username: Self::text(row, "username"),
                    is_enabled: Self::flag(row, "is_enabled"),
                    created: Self::text(row, "created"),
                    roles,
                    groups,
                }
            })
            .collect();
        users.sort_by_key(|user| user.id);
        users
    }

    fn reverse_map(&self, _value: Vec<UserView>) -> Vec<MappedRow> {
        panic!("reverse_map is not supported")
    }
}

impl UserViewMapper {
    fn children(row: &MappedRow, name: &str) -> Vec<ChildView> {
        Self::many(row, name)
            .iter()
            .map(|child| ChildView {
                id: Self::id(child, "id"),
                name: Self::text(child, "name"),
                is_enabled: Self::flag(child, "is_enabled"),
            })
            .collect()
    }

    fn many(row: &MappedRow, name: &str) -> Vec<MappedRow> {
        match row.get(name) {
            Some(MappedValue::Many(items)) => items.clone(),
            _ => vec![],
        }
    }

    fn cell<'a>(row: &'a MappedRow, name: &str) -> Option<&'a Value> {
        match row.get(name) {
            Some(MappedValue::Cell(value)) => Some(value),
            _ => None,
        }
    }

    fn id(row: &MappedRow, name: &str) -> i64 {
        match Self::cell(row, name) {
            Some(Value::Int(number)) => *number,
            Some(Value::Text(text)) => text.parse().unwrap_or(0),
            _ => 0,
        }
    }

    fn text(row: &MappedRow, name: &str) -> String {
        match Self::cell(row, name) {
            Some(Value::Text(value)) => value.clone(),
            Some(Value::DateTime(date)) => Value::iso_datetime(*date),
            Some(Value::Null) | None => String::new(),
            Some(value) => value.wire_text(),
        }
    }

    fn flag(row: &MappedRow, name: &str) -> bool {
        match Self::cell(row, name) {
            Some(Value::Bool(value)) => *value,
            Some(Value::Int(number)) => *number != 0,
            _ => false,
        }
    }
}

fn created() -> Parameter {
    Parameter::new("created", Utc::now())
}

fn updated(name: &str) -> Parameter {
    Parameter::new(name, vec![0_u8])
}
