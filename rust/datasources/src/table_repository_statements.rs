use crate::builders::StatementBuilder;
use crate::parameter::Parameter;
use crate::statement::Statement;
use crate::where_trait::Where;

pub struct TableRepositoryStatements {
    table: String,
    columns: Vec<String>,
}

impl TableRepositoryStatements {
    pub fn new(table: impl Into<String>, columns: Vec<String>) -> Self {
        Self {
            table: table.into(),
            columns,
        }
    }

    pub fn query(&self, where_clause: Option<&dyn Where>) -> (String, Vec<Parameter>) {
        let refs: Vec<&str> = self.columns.iter().map(String::as_str).collect();
        let built = StatementBuilder::create_query(&self.table, &refs, where_clause);
        (
            built.get_text(),
            where_clause
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| built.parameters()),
        )
    }

    pub fn insert(&self, fields: &[Parameter]) -> String {
        StatementBuilder::create_insert(&self.table, fields).get_text()
    }

    pub fn update(&self, fields: &[Parameter], where_clause: &dyn Where) -> String {
        StatementBuilder::create_update(&self.table, fields, Some(where_clause)).get_text()
    }

    pub fn delete(&self, where_clause: &dyn Where) -> String {
        StatementBuilder::create_delete(&self.table, Some(where_clause)).get_text()
    }
}
