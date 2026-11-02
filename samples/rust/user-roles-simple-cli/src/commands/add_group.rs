use crate::app::App;
use crate::columns;
use crate::commands::write_row;
use datasources::{Parameter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddGroup {
    pub name: String,
}

impl AddGroup {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        let column = columns::map("name");
        write_row::execute(
            app.command(),
            app.provider(),
            "groups",
            "add_group",
            &[
                Parameter::new(column.as_str(), self.name.as_str()),
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
