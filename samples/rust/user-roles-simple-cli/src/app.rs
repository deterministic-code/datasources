use crate::commands::CliCommand;
use crate::password_hash;
use chrono::Utc;
use datasources::{Command, Result, Value};

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
}
