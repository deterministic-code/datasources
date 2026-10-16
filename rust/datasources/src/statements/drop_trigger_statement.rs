use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct DropTriggerStatement {
    name: String,
    table: String,
}

impl DropTriggerStatement {
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

impl Statement for DropTriggerStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_drop_trigger(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for DropTriggerStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
