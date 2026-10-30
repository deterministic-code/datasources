use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;
use crate::SpecField;

#[derive(Clone, Debug)]
pub struct SqlSeedColumn {
    pub column: String,
    pub field: SpecField,
    pub value: Option<String>,
}

pub struct SqlSeedStatement {
    table: String,
    columns: Vec<SqlSeedColumn>,
    kind: String,
    identity: bool,
    id_column: Option<String>,
}

impl SqlSeedStatement {
    pub fn new(
        table: impl AsRef<str>,
        columns: Vec<SqlSeedColumn>,
        kind: impl AsRef<str>,
        identity: bool,
        id_column: Option<String>,
    ) -> Self {
        Self {
            table: table.as_ref().to_string(),
            columns,
            kind: kind.as_ref().to_string(),
            identity,
            id_column,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn columns(&self) -> &[SqlSeedColumn] {
        &self.columns
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn identity(&self) -> bool {
        self.identity
    }

    pub fn id_column(&self) -> Option<&str> {
        self.id_column.as_deref()
    }
}

impl Statement for SqlSeedStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_sql_seed(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for SqlSeedStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
