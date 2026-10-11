use super::bulk_insert_statement::flatten_bulk_parameters;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use crate::parameter::Parameter;

pub struct BulkDeleteStatement {
    table: String,
    ids: Vec<String>,
    rows: Vec<Vec<Parameter>>,
}

impl BulkDeleteStatement {
    pub fn new(table: impl AsRef<str>, ids: &[String], rows: Vec<Vec<Parameter>>) -> Self {
        Self {
            table: table.as_ref().to_string(),
            ids: ids.to_vec(),
            rows,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn ids(&self) -> &[String] {
        &self.ids
    }

    pub fn rows(&self) -> &[Vec<Parameter>] {
        &self.rows
    }

    pub fn parameters(&self) -> Vec<Parameter> {
        flatten_bulk_parameters(&self.ids, &self.rows)
    }
}

impl crate::ReadInput for BulkDeleteStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for BulkDeleteStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_bulk_delete(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Delete
    }
}
