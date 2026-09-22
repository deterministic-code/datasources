use crate::app::App;
use crate::commands::set_user_enabled;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisableUser {
    pub username: String,
}

impl DisableUser {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        set_user_enabled::execute(app, &self.username, false).await
    }
}
