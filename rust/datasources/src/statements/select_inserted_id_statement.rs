use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct SelectInsertedIdStatement {
    pub table: String,
    pub column: String,
}

impl SelectInsertedIdStatement {
    pub fn new(table: impl Into<String>, column: impl Into<String>) -> Self {
        Self {
            table: table.into(),
            column: column.into(),
        }
    }
}

impl Statement for SelectInsertedIdStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_select_inserted_id(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
