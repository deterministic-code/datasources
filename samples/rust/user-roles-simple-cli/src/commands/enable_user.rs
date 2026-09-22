use crate::app::App;
use crate::commands::set_user_enabled;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnableUser {
    pub username: String,
}

impl EnableUser {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        set_user_enabled::execute(app, &self.username, true).await
    }
}
