use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use crate::parameter::Parameter;

pub struct InsertStatement {
    table: String,
    fields: Vec<Parameter>,
}

impl InsertStatement {
    pub fn new(table: impl AsRef<str>, fields: &[Parameter]) -> Self {
        Self {
            table: table.as_ref().to_string(),
            fields: fields.to_vec(),
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn fields(&self) -> &[Parameter] {
        &self.fields
    }
}

impl crate::ReadInput for InsertStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for InsertStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_insert(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Insert
    }
}
