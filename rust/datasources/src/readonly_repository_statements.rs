use crate::IWhere;
use crate::statement_mappers::StatementMappers;
use crate::{SelectStatement, SelectStatementOptions};

pub struct ReadonlyRepositoryStatements {
    table: String,
    columns: Vec<String>,
    mappings: StatementMappers,
}

impl ReadonlyRepositoryStatements {
    pub fn new(table: impl Into<String>, columns: Vec<String>, mappings: StatementMappers) -> Self {
        Self {
            table: table.into(),
            columns,
            mappings,
        }
    }

    pub fn mappings(&self) -> StatementMappers {
        self.mappings.clone()
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn read_columns(&self) -> &[String] {
        &self.columns
    }

    pub fn select(&self, where_clause: Option<&dyn IWhere>) -> SelectStatement {
        let refs: Vec<&str> = self.columns.iter().map(String::as_str).collect();
        SelectStatement::new(SelectStatementOptions {
            where_clause,
            ..SelectStatementOptions::new(&self.table, &refs)
        })
    }
}
