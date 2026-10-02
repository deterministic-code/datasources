use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct CreateIndexStatement {
    name: String,
    table: String,
    columns: Vec<String>,
    unique: bool,
}

impl CreateIndexStatement {
    pub fn new(
        name: impl AsRef<str>,
        table: impl AsRef<str>,
        columns: &[&str],
        unique: bool,
    ) -> Self {
        Self {
            name: name.as_ref().to_string(),
            table: table.as_ref().to_string(),
            columns: columns.iter().map(|column| (*column).to_string()).collect(),
            unique,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn unique(&self) -> bool {
        self.unique
    }
}

impl Statement for CreateIndexStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_create_index(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for CreateIndexStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
