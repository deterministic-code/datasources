use crate::app::App;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddRole {
    pub name: String,
}

impl AddRole {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        app.write_named("roles", "name", &self.name, "add_role").await
    }
}
