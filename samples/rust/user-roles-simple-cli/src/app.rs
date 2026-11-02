use crate::commands::CliCommand;
use crate::password_hash;
use datasources::{Command, Result};

pub struct App<'a> {
    command: &'a dyn Command,
    provider: String,
    hash: Box<dyn Fn(&str) -> Result<String> + Send + Sync>,
    procedures: bool,
}

impl<'a> App<'a> {
    pub fn new(command: &'a dyn Command, provider: impl Into<String>) -> Self {
        Self {
            command,
            provider: provider.into(),
            hash: Box::new(password_hash::hash),
            procedures: true,
        }
    }

    pub fn procedures(mut self, procedures: bool) -> Self {
        self.procedures = procedures;
        self
    }

    pub fn with_hash(
        mut self,
        hash: impl Fn(&str) -> Result<String> + Send + Sync + 'static,
    ) -> Self {
        self.hash = Box::new(hash);
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

    pub(crate) fn use_procedures(&self) -> bool {
        self.procedures
    }

    pub(crate) fn hash_password(&self, password: &str) -> Result<String> {
        (self.hash)(password)
    }
}
