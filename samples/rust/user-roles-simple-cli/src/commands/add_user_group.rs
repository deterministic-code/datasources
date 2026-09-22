use crate::app::App;
use crate::columns;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddUserGroup {
    pub username: String,
    pub group: String,
}

impl AddUserGroup {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        app.write_link(
            &self.username,
            "groups",
            &columns::map("name"),
            &self.group,
            "user_groups",
            "group_id",
            "add_user_group",
        )
        .await
    }
}
