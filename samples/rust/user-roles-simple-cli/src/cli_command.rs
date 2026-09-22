#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    AddUser { username: String, password: String },
    AddRole { name: String },
    AddGroup { name: String },
    AddUserRole { username: String, role: String },
    AddUserGroup { username: String, group: String },
    EnableUser { username: String },
    DisableUser { username: String },
    ListUsersRolesGroups,
}
