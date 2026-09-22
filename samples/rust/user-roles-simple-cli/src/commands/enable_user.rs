use crate::app::App;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnableUser {
    pub username: String,
}

impl EnableUser {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        app.set_enabled(&self.username, true).await
    }
}
