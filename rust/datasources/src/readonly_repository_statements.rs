use crate::i_where::IWhere;
use crate::statement_mappers::StatementMappers;
use crate::statements::Text;
use crate::where_text::WhereText;
use crate::{SelectStatement, SelectStatementOptions, Where};

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
        let built = SelectStatement::new(SelectStatementOptions {
            where_clause,
            ..SelectStatementOptions::new(&self.table, &refs)
        });
        match where_clause.and_then(|clause| clause.as_any().downcast_ref::<Where>()) {
            Some(clause) => SelectStatement::with_where(built.get_text(), clause.clone()),
            None => match where_clause {
                Some(predicate) => SelectStatement::with_where(
                    built.get_text(),
                    WhereText::new(predicate.get_text()),
                ),
                None => SelectStatement::text(built.get_text()),
            },
        }
    }
}
