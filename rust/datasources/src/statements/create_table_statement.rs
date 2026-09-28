use super::create_table_statement_renderer;
use super::statement::Statement;
use super::table_constraint::TableConstraint;
use super::text::Text;
use crate::builders::Column;

pub struct CreateTableStatement {
    text: Option<String>,
    pub table: Option<String>,
    pub columns: Vec<Column>,
    pub constraints: Vec<TableConstraint>,
}

impl CreateTableStatement {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            table: None,
            columns: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn new(
        table: impl AsRef<str>,
        columns: &[Column],
        constraints: &[TableConstraint],
    ) -> Self {
        Self {
            text: None,
            table: Some(table.as_ref().to_string()),
            columns: columns.to_vec(),
            constraints: constraints.to_vec(),
        }
    }
}

impl Text for CreateTableStatement {
    fn get_text(&self) -> String {
        self.text
            .clone()
            .unwrap_or_else(|| create_table_statement_renderer::render(self))
    }
}

impl Statement for CreateTableStatement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateTableStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
