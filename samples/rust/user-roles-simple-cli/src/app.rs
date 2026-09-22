use crate::as_user_view;
use crate::cli_command::CliCommand;
use crate::columns;
use crate::password_hash;
use crate::schema;
use crate::users_roles_groups_query;
use crate::users_view;
use datasources::{
    collect_reader, Command, DatasourceError, ExecuteStoredProcedureStatement, FlatToNestedMapper,
    InsertStatement, Mapper, Operator, Parameter, QueryStatement, Result, UpdateStatement,
    Value, Where, WhereClause,
};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct App<'a> {
    command: &'a dyn Command,
    provider: String,
    now: Box<dyn Fn() -> String + Send + Sync>,
    hash: Box<dyn Fn(&str) -> Result<String> + Send + Sync>,
    token: Box<dyn Fn() -> Vec<u8> + Send + Sync>,
}

impl<'a> App<'a> {
    pub fn new(command: &'a dyn Command, provider: impl Into<String>) -> Self {
        Self {
            command,
            provider: provider.into(),
            now: Box::new(|| {
                let millis = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|value| value.as_millis())
                    .unwrap_or(0);
                format!("{millis}")
            }),
            hash: Box::new(password_hash::hash),
            token: Box::new(|| vec![1, 2, 3, 4]),
        }
    }

    pub fn with_clock(
        mut self,
        now: impl Fn() -> String + Send + Sync + 'static,
        hash: impl Fn(&str) -> Result<String> + Send + Sync + 'static,
        token: impl Fn() -> Vec<u8> + Send + Sync + 'static,
    ) -> Self {
        self.now = Box::new(now);
        self.hash = Box::new(hash);
        self.token = Box::new(token);
        self
    }

    pub async fn run(&self, parsed: CliCommand) -> Result<String> {
        match parsed {
            CliCommand::AddUser { username, password } => self.write_user(&username, &password).await,
            CliCommand::AddRole { name } => self.write_named("roles", "name", &name, "add_role").await,
            CliCommand::AddGroup { name } => {
                self.write_named("groups", &columns::map("name"), &name, "add_group")
                    .await
            }
            CliCommand::AddUserRole { username, role } => {
                self.write_link(
                    &username,
                    "roles",
                    "name",
                    &role,
                    "user_roles",
                    &columns::map("role_id"),
                    "add_user_role",
                )
                .await
            }
            CliCommand::AddUserGroup { username, group } => {
                self.write_link(
                    &username,
                    "groups",
                    &columns::map("name"),
                    &group,
                    "user_groups",
                    "group_id",
                    "add_user_group",
                )
                .await
            }
            CliCommand::EnableUser { username } => self.set_enabled(&username, true).await,
            CliCommand::DisableUser { username } => self.set_enabled(&username, false).await,
            CliCommand::ListUsersRolesGroups => self.list().await,
        }
    }

    async fn write_user(&self, username: &str, password: &str) -> Result<String> {
        let created = (self.now)();
        let updated = (self.token)();
        let hashed = (self.hash)(password)?;
        self.write(
            &self.table("users"),
            "add_user",
            &[
                Parameter::new("username", username),
                Parameter::new("password", hashed.as_str()),
                Parameter::new("is_enabled", true),
                Parameter::new("created", created.as_str()),
                Parameter::new("updated", updated.clone()),
            ],
            &[
                Parameter::new("p_username", username),
                Parameter::new("p_password", hashed.as_str()),
                Parameter::new("p_is_enabled", true),
                Parameter::new("p_created", created.as_str()),
                Parameter::new("p_updated", updated),
            ],
        )
        .await
    }

    async fn write_named(
        &self,
        table: &str,
        column: &str,
        value: &str,
        procedure: &str,
    ) -> Result<String> {
        let created = (self.now)();
        let updated = (self.token)();
        self.write(
            &self.table(table),
            procedure,
            &[
                Parameter::new(column, value),
                Parameter::new("is_enabled", true),
                Parameter::new("created", created.as_str()),
                Parameter::new("updated", updated.clone()),
            ],
            &[
                Parameter::new("p_name", value),
                Parameter::new("p_is_enabled", true),
                Parameter::new("p_created", created.as_str()),
                Parameter::new("p_updated", updated),
            ],
        )
        .await
    }

    async fn write_link(
        &self,
        username: &str,
        right_table: &str,
        right_column: &str,
        right_value: &str,
        link_table: &str,
        link_column: &str,
        procedure: &str,
    ) -> Result<String> {
        let user_id = self.id(&self.table("users"), "username", username).await?;
        let right_id = self.id(&self.table(right_table), right_column, right_value).await?;
        let created = (self.now)();
        let updated = (self.token)();
        let insert = [
            Parameter::new("user_id", user_id),
            Parameter::new(link_column, right_id),
            Parameter::new("created", created.as_str()),
            Parameter::new("updated", updated.clone()),
        ];
        let call = [
            Parameter::new("p_user_id", user_id),
            Parameter::new(
                if link_column == "bad_role_id" {
                    "p_role_id"
                } else {
                    "p_group_id"
                },
                right_id,
            ),
            Parameter::new("p_created", created.as_str()),
            Parameter::new("p_updated", updated),
        ];
        if self.provider == "mysql" {
            self.command
                .execute_statement(&ExecuteStoredProcedureStatement::new(procedure, &call), Some(&call))
                .await?;
        } else {
            self.command
                .execute_statement(&InsertStatement::new(self.table(link_table), &insert), Some(&insert))
                .await?;
        }
        Ok(String::new())
    }

    async fn set_enabled(&self, username: &str, enabled: bool) -> Result<String> {
        let updated = (self.token)();
        if self.provider == "mysql" {
            let fields = [
                Parameter::new("p_username", username),
                Parameter::new("p_is_enabled", enabled),
                Parameter::new("p_updated", updated),
            ];
            self.command
                .execute_statement(
                    &ExecuteStoredProcedureStatement::new("set_user_enabled", &fields),
                    Some(&fields),
                )
                .await?;
            return Ok(String::new());
        }
        let id = self.id(&self.table("users"), "username", username).await?;
        let fields = [
            Parameter::new("is_enabled", enabled),
            Parameter::new("updated", updated),
        ];
        let where_clause = WhereClause::new(Operator::equals("id", id));
        let mut parameters = fields.to_vec();
        parameters.extend(where_clause.parameters());
        self.command
            .execute_statement(
                &UpdateStatement::new(self.table("users"), &fields, Some(&where_clause)),
                Some(&parameters),
            )
            .await?;
        Ok(String::new())
    }

    async fn list(&self) -> Result<String> {
        let query = users_roles_groups_query::create(&self.provider);
        let reader = self
            .command
            .execute_reader_statement(&query, None)
            .await?;
        let rows = collect_reader(reader.as_ref()).await?;
        let mapped = Mapper::map(&FlatToNestedMapper::new(users_view::type_map()), rows);
        Ok(serde_json::to_string(&as_user_view::from_mapped(&mapped))
            .map_err(|error| DatasourceError::message(error.to_string()))?)
    }

    async fn id(&self, table: &str, column: &str, value: &str) -> Result<i64> {
        let where_clause = WhereClause::new(Operator::equals(column, value));
        let query = QueryStatement::new(table, &["id"], Some(&where_clause), &[]);
        let parameters = where_clause.parameters();
        let reader = self
            .command
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

    async fn write(
        &self,
        table: &str,
        procedure: &str,
        insert: &[Parameter],
        call: &[Parameter],
    ) -> Result<String> {
        if self.provider == "mysql" {
            self.command
                .execute_statement(&ExecuteStoredProcedureStatement::new(procedure, call), Some(call))
                .await?;
        } else {
            self.command
                .execute_statement(&InsertStatement::new(table, insert), Some(insert))
                .await?;
        }
        Ok(String::new())
    }

    fn table(&self, name: &str) -> String {
        schema::ident(&self.provider, name)
    }
}
