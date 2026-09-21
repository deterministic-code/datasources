use crate::mappers::StatementMapper;
use crate::parameter::Parameter;
use crate::statements::{DeleteStatement, InsertStatement, Statement, Text, UpdateStatement};
use crate::QueryStatement;
use crate::where_trait::Where;
use std::sync::Arc;

pub struct RepositoryStatements {
    table: String,
    columns: Vec<String>,
    mapper: Arc<StatementMapper>,
}

impl RepositoryStatements {
    pub fn new(table: impl Into<String>, columns: Vec<String>) -> Self {
        Self::with_mapper(table, columns, Arc::new(StatementMapper::identity("", "")))
    }

    pub fn with_mapper(
        table: impl Into<String>,
        columns: Vec<String>,
        mapper: Arc<StatementMapper>,
    ) -> Self {
        Self {
            table: table.into(),
            columns,
            mapper,
        }
    }

    pub fn query(&self, where_clause: Option<&dyn Where>) -> (String, Vec<Parameter>) {
        let refs: Vec<&str> = self.columns.iter().map(String::as_str).collect();
        let built = QueryStatement::new(&self.table, &refs, where_clause, &[]);
        let text = StatementMapper::map_sql(&self.mapper.query, built.get_text());
        (
            text,
            where_clause
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| built.parameters()),
        )
    }

    pub fn insert(&self, fields: &[Parameter]) -> String {
        StatementMapper::map_sql(
            &self.mapper.insert,
            InsertStatement::new(&self.table, fields).get_text(),
        )
    }

    pub fn update(&self, fields: &[Parameter], where_clause: &dyn Where) -> String {
        StatementMapper::map_sql(
            &self.mapper.update,
            UpdateStatement::new(&self.table, fields, Some(where_clause)).get_text(),
        )
    }

    pub fn delete(&self, where_clause: &dyn Where) -> String {
        StatementMapper::map_sql(
            &self.mapper.delete,
            DeleteStatement::new(&self.table, Some(where_clause)).get_text(),
        )
    }
}
