use crate::app::App;
use crate::columns;
use crate::commands::write_link;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddUserRole {
    pub username: String,
    pub role: String,
}

impl AddUserRole {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        write_link::execute(
            app,
            &self.username,
            "roles",
            "name",
            &self.role,
            "user_roles",
            &columns::map("role_id"),
            "add_user_role",
        )
        .await
    }
}
