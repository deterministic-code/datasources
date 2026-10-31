use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use super::i_where::IWhere;
use super::where_text::boxed;

pub struct DeleteStatement {
    table: String,
    where_clause: Option<Box<dyn IWhere>>,
}

impl DeleteStatement {
    pub fn new(table: impl AsRef<str>, where_clause: Option<&dyn IWhere>) -> Self {
        Self {
            table: table.as_ref().to_string(),
            where_clause: boxed(where_clause),
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn where_clause(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}

impl crate::ReadInput for DeleteStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}

impl Statement for DeleteStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_delete(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn kind(&self) -> super::statement::StatementKind {
        super::statement::StatementKind::Delete
    }
    fn predicate(&self) -> Option<&dyn IWhere> {
        self.where_clause.as_deref()
    }
}
