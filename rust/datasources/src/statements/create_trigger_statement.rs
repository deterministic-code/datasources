use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct CreateTriggerStatement {
    table: String,
    name: String,
    updated_column: String,
    key_column: String,
    utc_now: String,
}

impl CreateTriggerStatement {
    pub fn new(
        table: impl AsRef<str>,
        name: impl AsRef<str>,
        updated_column: impl AsRef<str>,
        key_column: impl AsRef<str>,
        utc_now: impl AsRef<str>,
    ) -> Self {
        Self {
            table: table.as_ref().to_string(),
            name: name.as_ref().to_string(),
            updated_column: updated_column.as_ref().to_string(),
            key_column: key_column.as_ref().to_string(),
            utc_now: utc_now.as_ref().to_string(),
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn updated_column(&self) -> &str {
        &self.updated_column
    }

    pub fn key_column(&self) -> &str {
        &self.key_column
    }

    pub fn utc_now(&self) -> &str {
        &self.utc_now
    }
}

impl Statement for CreateTriggerStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_create_trigger(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateTriggerStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
