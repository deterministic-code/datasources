use crate::crud_mapper::CRUDMapper;
use crate::parameter::Parameter;
use crate::statements::{DeleteStatement, InsertStatement, Text, UpdateStatement};
use crate::text_where::TextWhere;
use crate::QueryStatement;
use crate::where_trait::Where;
use std::sync::Arc;

pub struct RepositoryStatements {
    table: String,
    columns: Vec<String>,
    mapper: Arc<CRUDMapper>,
}

impl RepositoryStatements {
    pub fn new(table: impl Into<String>, columns: Vec<String>) -> Self {
        Self::with_mapper(table, columns, Arc::new(CRUDMapper::identity("", "")))
    }

    pub fn with_mapper(
        table: impl Into<String>,
        columns: Vec<String>,
        mapper: Arc<CRUDMapper>,
    ) -> Self {
        Self {
            table: table.into(),
            columns,
            mapper,
        }
    }

    pub fn query(&self, where_clause: Option<&dyn Where>) -> QueryStatement {
        let refs: Vec<&str> = self.columns.iter().map(String::as_str).collect();
        let built = QueryStatement::new(&self.table, &refs, where_clause, &[]);
        let text = CRUDMapper::map_sql(&self.mapper.query, built.get_text());
        match where_clause {
            Some(predicate) => QueryStatement::with_where(text, TextWhere::new(predicate.get_text())),
            None => QueryStatement::text(text),
        }
    }

    pub fn insert(&self, fields: &[Parameter]) -> InsertStatement {
        InsertStatement::text(CRUDMapper::map_sql(
            &self.mapper.insert,
            InsertStatement::new(&self.table, fields).get_text(),
        ))
    }

    pub fn update(&self, fields: &[Parameter], where_clause: &dyn Where) -> UpdateStatement {
        UpdateStatement::text(
            CRUDMapper::map_sql(
                &self.mapper.update,
                UpdateStatement::new(&self.table, fields, Some(where_clause)).get_text(),
            ),
            Some(Box::new(TextWhere::new(where_clause.get_text()))),
        )
    }

    pub fn delete(&self, where_clause: &dyn Where) -> DeleteStatement {
        DeleteStatement::raw(
            CRUDMapper::map_sql(
                &self.mapper.delete,
                DeleteStatement::new(&self.table, Some(where_clause)).get_text(),
            ),
            Some(Box::new(TextWhere::new(where_clause.get_text()))),
        )
    }
}
