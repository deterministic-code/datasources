pub mod add_group;
pub mod add_role;
pub mod add_user;
pub mod add_user_group;
pub mod add_user_role;
pub mod disable_user;
pub mod enable_user;
pub mod list_users_roles_groups;

pub use add_group::AddGroup;
pub use add_role::AddRole;
pub use add_user::AddUser;
pub use add_user_group::AddUserGroup;
pub use add_user_role::AddUserRole;
pub use disable_user::DisableUser;
pub use enable_user::EnableUser;
pub use list_users_roles_groups::ListUsersRolesGroups;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    AddUser(AddUser),
    AddRole(AddRole),
    AddGroup(AddGroup),
    AddUserRole(AddUserRole),
    AddUserGroup(AddUserGroup),
    EnableUser(EnableUser),
    DisableUser(DisableUser),
    ListUsersRolesGroups(ListUsersRolesGroups),
}
