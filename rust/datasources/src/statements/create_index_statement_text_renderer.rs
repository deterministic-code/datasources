use super::create_index_statement::CreateIndexStatement;

pub struct CreateIndexStatementTextRenderer;

impl CreateIndexStatementTextRenderer {
    pub fn render(&self, statement: &CreateIndexStatement) -> String {
        format!(
            "CREATE {}INDEX {} ON {} ({})",
            if statement.unique() { "UNIQUE " } else { "" },
            self.quote(statement.name()),
            self.quote(statement.table()),
            statement
                .columns()
                .iter()
                .map(|column| self.quote(column))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn quote(&self, name: &str) -> String {
        name.to_string()
    }
}

impl Default for CreateIndexStatementTextRenderer {
    fn default() -> Self {
        Self
    }
}
