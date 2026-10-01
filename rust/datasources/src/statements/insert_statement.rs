use super::statement::Statement;
use super::text::Text;
use crate::parameter::Parameter;

pub struct InsertStatement {
    text: Option<String>,
    table: Option<String>,
    fields: Vec<Parameter>,
}

impl InsertStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            table: None,
            fields: Vec::new(),
        }
    }

    pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self {
        Self {
            text: None,
            table: Some(table.as_ref().to_string()),
            fields: fields.to_vec(),
        }
    }

    pub fn table(&self) -> Option<&str> {
        self.table.as_deref()
    }

    pub fn fields(&self) -> &[Parameter] {
        &self.fields
    }
}

impl Text for InsertStatement {
    fn get_text(&self) -> String {
        match &self.text {
            Some(text) => text.clone(),
            None => format!(
                "INSERT INTO {} ({}) VALUES ({})",
                self.table.as_deref().unwrap_or(""),
                self.fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                self.fields
                    .iter()
                    .map(|field| format!("${}", field.name))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl crate::ReadInput for InsertStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for InsertStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Insert
    }
}
