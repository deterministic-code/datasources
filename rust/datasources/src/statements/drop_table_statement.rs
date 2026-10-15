use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct DropTableStatement {
    table: String,
}

impl DropTableStatement {
    pub fn new(table: impl AsRef<str>) -> Self {
        Self {
            table: table.as_ref().to_string(),
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }
}

impl Statement for DropTableStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_drop_table(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for DropTableStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
