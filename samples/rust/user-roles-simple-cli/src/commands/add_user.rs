use crate::app::App;
use crate::commands::write_row;
use datasources::{Parameter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddUser {
    pub username: String,
    pub password: String,
}

impl AddUser {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        let hashed = app.hash_password(&self.password)?;
        write_row::execute(
            app.command(),
            app.provider(),
            "users",
            "add_user",
            &[
                Parameter::new("username", self.username.as_str()),
                Parameter::new("password", hashed.as_str()),
                Parameter::new("is_enabled", true),
            ],
            &[
                Parameter::new("p_username", self.username.as_str()),
                Parameter::new("p_password", hashed.as_str()),
                Parameter::new("p_is_enabled", true),
            ],
            app.use_procedures(),
        )
        .await
    }
}
