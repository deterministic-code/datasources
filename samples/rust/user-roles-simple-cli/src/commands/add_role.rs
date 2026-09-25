use crate::app::App;
use crate::commands::write_row;
use crate::schema;
use datasources::{Parameter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddRole {
    pub name: String,
}

impl AddRole {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        let created = app.stamp();
        let updated = app.token_bytes();
        write_row::execute(
            app.command(),
            app.provider(),
            &schema::ident(app.provider(), "roles"),
            "add_role",
            &[
                Parameter::new("name", self.name.as_str()),
                Parameter::new("is_enabled", schema::flag(app.provider(), true)),
                Parameter::new("created", created.as_str()),
                Parameter::new("updated", updated.clone()),
            ],
            &[
                Parameter::new("p_name", self.name.as_str()),
                Parameter::new("p_is_enabled", true),
                Parameter::new("p_created", created.as_str()),
                Parameter::new("p_updated", updated),
            ],
            app.use_procedures(),
        )
        .await
    }
}
