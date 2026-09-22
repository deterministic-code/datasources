use crate::group_view::GroupView;
use crate::role_view::RoleView;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserView {
    pub id: i64,
    pub username: String,
    pub is_enabled: bool,
    pub created: String,
    pub roles: Vec<RoleView>,
    pub groups: Vec<GroupView>,
}
