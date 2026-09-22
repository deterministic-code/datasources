use crate::app::App;
use crate::columns;
use datasources::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddGroup {
    pub name: String,
}

impl AddGroup {
    pub async fn run(&self, app: &App<'_>) -> Result<String> {
        app.write_named("groups", &columns::map("name"), &self.name, "add_group")
            .await
    }
}
