use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;

pub struct InsertStatement {
    text: String,
}

impl InsertStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self {
        Self {
            text: format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table.as_ref(),
                fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                fields
                    .iter()
                    .map(|field| format!("${}", field.name))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl Text for InsertStatement {
    fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl crate::ReadInput for InsertStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for InsertStatement {
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Insert
    }
}
