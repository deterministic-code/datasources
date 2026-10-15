use super::insert_statement::InsertStatement;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct IdentityInsertStatement {
    table: String,
    id_column: String,
    inserts: Vec<InsertStatement>,
    identity: bool,
}

impl IdentityInsertStatement {
    pub fn new(
        table: impl AsRef<str>,
        id_column: impl AsRef<str>,
        inserts: Vec<InsertStatement>,
        identity: bool,
    ) -> Self {
        Self {
            table: table.as_ref().to_string(),
            id_column: id_column.as_ref().to_string(),
            inserts,
            identity,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn id_column(&self) -> &str {
        &self.id_column
    }

    pub fn inserts(&self) -> &[InsertStatement] {
        &self.inserts
    }

    pub fn identity(&self) -> bool {
        self.identity
    }
}

impl Statement for IdentityInsertStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_identity_insert(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for IdentityInsertStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
