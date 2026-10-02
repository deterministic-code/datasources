use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use crate::builders::Column;
use super::table_constraint::TableConstraint;

pub struct CreateTableStatement {
    pub table: String,
    pub columns: Vec<Column>,
    pub constraints: Vec<TableConstraint>,
}

impl CreateTableStatement {
    pub fn new(
        table: impl AsRef<str>,
        columns: &[Column],
        constraints: &[TableConstraint],
    ) -> Self {
        Self {
            table: table.as_ref().to_string(),
            columns: columns.to_vec(),
            constraints: constraints.to_vec(),
        }
    }
}

impl Statement for CreateTableStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_create_table(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateTableStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
