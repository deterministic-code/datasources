use crate::app::App;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisableUser {
    pub username: String,
}

impl DisableUser {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        app.set_enabled(&self.username, false).await
    }
}
