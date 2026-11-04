use super::delete_statement::DeleteStatement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct DeleteStatementTextRenderer<'a> {
    text: &'a StatementTextRenderer,
    quote: fn(&str) -> String,
}

impl<'a> DeleteStatementTextRenderer<'a> {
    pub fn new(text: &'a StatementTextRenderer) -> Self {
        Self {
            text,
            quote: |name| name.to_string(),
        }
    }

    pub fn with_quote(mut self, quote: fn(&str) -> String) -> Self {
        self.quote = quote;
        self
    }

    pub fn render(&self, statement: &DeleteStatement) -> String {
        match statement.where_clause() {
            None => format!("DELETE FROM {}", self.quote(statement.table())),
            Some(predicate) => format!(
                "DELETE FROM {} WHERE {}",
                self.quote(statement.table()),
                self.text.render_where(predicate)
            ),
        }
    }

    pub fn quote(&self, name: &str) -> String {
        (self.quote)(name)
    }
}
