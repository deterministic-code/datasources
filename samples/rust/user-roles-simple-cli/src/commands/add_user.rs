use crate::app::App;
use crate::commands::write_row;
use crate::schema;
use datasources::{Parameter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddUser {
    pub username: String,
    pub password: String,
}

impl AddUser {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        let created = app.stamp();
        let updated = app.token_bytes();
        let hashed = app.hash_password(&self.password)?;
        write_row::execute(
            app.command(),
            app.provider(),
            &schema::ident(app.provider(), "users"),
            "add_user",
            &[
                Parameter::new("username", self.username.as_str()),
                Parameter::new("password", hashed.as_str()),
                Parameter::new("is_enabled", true),
                Parameter::new("created", created.as_str()),
                Parameter::new("updated", updated.clone()),
            ],
            &[
                Parameter::new("p_username", self.username.as_str()),
                Parameter::new("p_password", hashed.as_str()),
                Parameter::new("p_is_enabled", true),
                Parameter::new("p_created", created.as_str()),
                Parameter::new("p_updated", updated),
            ],
        )
        .await
    }
}
