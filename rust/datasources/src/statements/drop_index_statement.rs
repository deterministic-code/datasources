use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct DropIndexStatement {
    name: String,
    table: String,
}

impl DropIndexStatement {
    pub fn new(name: impl AsRef<str>, table: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_string(),
            table: table.as_ref().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn table(&self) -> &str {
        &self.table
    }
}

impl Statement for DropIndexStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_drop_index(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for DropIndexStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
