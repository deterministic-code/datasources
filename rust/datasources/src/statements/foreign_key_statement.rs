use super::foreign_key::ReferentialAction;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

pub struct ForeignKeyStatement {
    table: String,
    name: String,
    columns: Vec<String>,
    references_table: String,
    references_columns: Vec<String>,
    kind: String,
    on_delete: Option<ReferentialAction>,
    on_update: Option<ReferentialAction>,
}

impl ForeignKeyStatement {
    pub fn new(
        table: impl Into<String>,
        name: impl Into<String>,
        columns: Vec<String>,
        references_table: impl Into<String>,
        references_columns: Vec<String>,
        kind: impl Into<String>,
    ) -> Self {
        Self {
            table: table.into(),
            name: name.into(),
            columns,
            references_table: references_table.into(),
            references_columns,
            kind: kind.into(),
            on_delete: None,
            on_update: None,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn references_table(&self) -> &str {
        &self.references_table
    }

    pub fn references_columns(&self) -> &[String] {
        &self.references_columns
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn on_delete(&self) -> Option<ReferentialAction> {
        self.on_delete
    }

    pub fn on_update(&self) -> Option<ReferentialAction> {
        self.on_update
    }
}

impl Statement for ForeignKeyStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_foreign_key(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl crate::ReadInput for ForeignKeyStatement {
    fn as_statement(&self) -> Option<&dyn crate::Statement> {
        Some(self)
    }
}
