use crate::commands::CliCommand;
use crate::password_hash;
use crate::schema;
use chrono::Utc;
use datasources::{
    Command, DatasourceError, ExecuteStoredProcedureStatement, InsertStatement, Operator,
    Parameter, QueryStatement, Result, UpdateStatement, Value, Where, WhereClause,
};

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
            now: Box::new(|| Value::iso_datetime(Utc::now())),
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

    pub fn with_now(mut self, now: impl Fn() -> String + Send + Sync + 'static) -> Self {
        self.now = Box::new(now);
        self
    }

    pub async fn run(&self, parsed: CliCommand) -> Result<String> {
        match parsed {
            CliCommand::AddUser(cmd) => cmd.run(self).await,
            CliCommand::AddRole(cmd) => cmd.run(self).await,
            CliCommand::AddGroup(cmd) => cmd.run(self).await,
            CliCommand::AddUserRole(cmd) => cmd.run(self).await,
            CliCommand::AddUserGroup(cmd) => cmd.run(self).await,
            CliCommand::EnableUser(cmd) => cmd.run(self).await,
            CliCommand::DisableUser(cmd) => cmd.run(self).await,
            CliCommand::ListUsersRolesGroups(cmd) => cmd.run(self).await,
        }
    }

    pub(crate) fn command(&self) -> &'a dyn Command {
        self.command
    }

    pub(crate) fn provider(&self) -> &str {
        &self.provider
    }

    pub(crate) fn stamp(&self) -> String {
        (self.now)()
    }

    pub(crate) fn hash_password(&self, password: &str) -> Result<String> {
        (self.hash)(password)
    }

    pub(crate) fn token_bytes(&self) -> Vec<u8> {
        (self.token)()
    }

    pub(crate) async fn write_named(
        &self,
        table: &str,
        column: &str,
        value: &str,
        procedure: &str,
    ) -> Result<String> {
        let created = self.stamp();
        let updated = self.token_bytes();
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

    pub(crate) async fn write_link(
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
        let created = self.stamp();
        let updated = self.token_bytes();
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

    pub(crate) async fn set_enabled(&self, username: &str, enabled: bool) -> Result<String> {
        let updated = self.token_bytes();
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

    pub(crate) async fn id(&self, table: &str, column: &str, value: &str) -> Result<i64> {
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

    pub(crate) async fn write(
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

    pub(crate) fn table(&self, name: &str) -> String {
        schema::ident(&self.provider, name)
    }
}
