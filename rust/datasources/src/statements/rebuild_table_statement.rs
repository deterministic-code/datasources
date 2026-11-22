use super::create_index_statement::CreateIndexStatement;
use super::create_table_statement::CreateTableStatement;
use super::create_trigger_statement::CreateTriggerStatement;
use super::statement::Statement;
use super::statement_text_renderer::StatementTextRenderer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildColumn {
    pub name: String,
    pub source: Option<String>,
    pub cast: Option<String>,
    pub literal: Option<String>,
}

impl RebuildColumn {
    pub fn copied(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: Some(source.into()),
            cast: None,
            literal: None,
        }
    }

    pub fn with_cast(mut self, cast: impl Into<String>) -> Self {
        self.cast = Some(cast.into());
        self
    }

    pub fn literal(name: impl Into<String>, literal: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: None,
            cast: None,
            literal: Some(literal.into()),
        }
    }

    pub fn null(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: None,
            cast: None,
            literal: None,
        }
    }
}

pub struct RebuildTableStatement {
    table: String,
    create: CreateTableStatement,
    columns: Vec<RebuildColumn>,
    indexes: Vec<CreateIndexStatement>,
    triggers: Vec<CreateTriggerStatement>,
}

impl RebuildTableStatement {
    pub fn new(
        table: impl Into<String>,
        create: CreateTableStatement,
        columns: Vec<RebuildColumn>,
        indexes: Vec<CreateIndexStatement>,
        triggers: Vec<CreateTriggerStatement>,
    ) -> Self {
        Self {
            table: table.into(),
            create,
            columns,
            indexes,
            triggers,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn create(&self) -> &CreateTableStatement {
        &self.create
    }

    pub fn columns(&self) -> &[RebuildColumn] {
        &self.columns
    }

    pub fn indexes(&self) -> &[CreateIndexStatement] {
        &self.indexes
    }

    pub fn triggers(&self) -> &[CreateTriggerStatement] {
        &self.triggers
    }
}

impl Statement for RebuildTableStatement {
    fn accept(&self, text: &StatementTextRenderer) -> String {
        text.statements().visit_rebuild_table(self, text)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
