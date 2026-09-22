use crate::app::App;
use crate::columns;
use crate::commands::write_row;
use crate::schema;
use datasources::{Parameter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddGroup {
    pub name: String,
}

impl AddGroup {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        let created = app.stamp();
        let updated = app.token_bytes();
        let column = columns::map("name");
        write_row::execute(
            app.command(),
            app.provider(),
            &schema::ident(app.provider(), "groups"),
            "add_group",
            &[
                Parameter::new(column.as_str(), self.name.as_str()),
                Parameter::new("is_enabled", true),
                Parameter::new("created", created.as_str()),
                Parameter::new("updated", updated.clone()),
            ],
            &[
                Parameter::new("p_name", self.name.as_str()),
                Parameter::new("p_is_enabled", true),
                Parameter::new("p_created", created.as_str()),
                Parameter::new("p_updated", updated),
            ],
        )
        .await
    }
}
