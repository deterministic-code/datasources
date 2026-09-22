use crate::commands::{
    AddGroup, AddRole, AddUser, AddUserGroup, AddUserRole, CliCommand, DisableUser, EnableUser,
    ListUsersRolesGroups,
};
use datasources::{DatasourceError, Result};

pub const USAGE: &str = "\
add-user <username> <password>
add-role <name>
add-group <name>
add-user-role <username> <role>
add-user-group <username> <group>
enable-user <username>
disable-user <username>
list-users-roles-groups";

pub fn args(args: &[String]) -> Result<CliCommand> {
    match args {
        [cmd, username, password] if cmd == "add-user" => Ok(CliCommand::AddUser(AddUser {
            username: username.clone(),
            password: password.clone(),
        })),
        [cmd, name] if cmd == "add-role" => Ok(CliCommand::AddRole(AddRole { name: name.clone() })),
        [cmd, name] if cmd == "add-group" => Ok(CliCommand::AddGroup(AddGroup { name: name.clone() })),
        [cmd, username, role] if cmd == "add-user-role" => Ok(CliCommand::AddUserRole(AddUserRole {
            username: username.clone(),
            role: role.clone(),
        })),
        [cmd, username, group] if cmd == "add-user-group" => {
            Ok(CliCommand::AddUserGroup(AddUserGroup {
                username: username.clone(),
                group: group.clone(),
            }))
        }
        [cmd, username] if cmd == "enable-user" => Ok(CliCommand::EnableUser(EnableUser {
            username: username.clone(),
        })),
        [cmd, username] if cmd == "disable-user" => Ok(CliCommand::DisableUser(DisableUser {
            username: username.clone(),
        })),
        [cmd] if cmd == "list-users-roles-groups" => {
            Ok(CliCommand::ListUsersRolesGroups(ListUsersRolesGroups))
        }
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
