use super::statement::Statement;
use super::text::Text;
use crate::builders::Column;

pub struct CreateTableStatement {
    text: String,
}

impl CreateTableStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(table: impl AsRef<str>, columns: &[Column]) -> Self {
        let cols = columns
            .iter()
            .map(|column| format!("{} {}", column.name, column.type_name))
            .collect::<Vec<_>>()
            .join(", ");
        Self {
            text: format!("CREATE TABLE {} ({})", table.as_ref(), cols),
        }
    }
}

impl Text for CreateTableStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl Statement for CreateTableStatement {}
