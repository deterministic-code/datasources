use crate::app::App;
use crate::commands::write_row;
use datasources::{Parameter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddRole {
    pub name: String,
}

impl AddRole {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        write_row::execute(
            app.command(),
            app.provider(),
            "roles",
            "add_role",
            &[
                Parameter::new("name", self.name.as_str()),
                Parameter::new("is_enabled", true),
            ],
            &[
                Parameter::new("p_name", self.name.as_str()),
                Parameter::new("p_is_enabled", true),
            ],
            app.use_procedures(),
        )
        .await
    }
}
