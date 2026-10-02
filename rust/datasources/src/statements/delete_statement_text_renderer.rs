use super::delete_statement::DeleteStatement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct DeleteStatementTextRenderer<'a> {
    text: &'a StatementTextRenderer,
}

impl<'a> DeleteStatementTextRenderer<'a> {
    pub fn new(text: &'a StatementTextRenderer) -> Self {
        Self { text }
    }

    pub fn render(&self, statement: &DeleteStatement) -> String {
        match statement.where_clause() {
            None => format!("DELETE FROM {}", statement.table()),
            Some(predicate) => format!(
                "DELETE FROM {} WHERE {}",
                statement.table(),
                self.text.render_where(predicate)
            ),
        }
    }
}
