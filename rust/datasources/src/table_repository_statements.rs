use crate::i_where::IWhere;
use crate::mappers::Mapper;
use crate::mappers::TransformMapper;
use crate::parameter::Parameter;
use crate::statements::{DeleteStatement, InsertStatement, Text, UpdateStatement};
use crate::where_text::WhereText;
use crate::{SelectStatement, SelectStatementOptions};
use std::sync::Arc;

pub struct RepositoryStatements {
    table: String,
    columns: Vec<String>,
    query: Option<Arc<TransformMapper<String, String>>>,
    insert: Option<Arc<TransformMapper<String, String>>>,
    update: Option<Arc<TransformMapper<String, String>>>,
    delete: Option<Arc<TransformMapper<String, String>>>,
}

impl RepositoryStatements {
    pub fn new(table: impl Into<String>, columns: Vec<String>) -> Self {
        Self::with_mappers(table, columns, None, None, None, None)
    }

    pub fn with_mappers(
        table: impl Into<String>,
        columns: Vec<String>,
        query: Option<Arc<TransformMapper<String, String>>>,
        insert: Option<Arc<TransformMapper<String, String>>>,
        update: Option<Arc<TransformMapper<String, String>>>,
        delete: Option<Arc<TransformMapper<String, String>>>,
    ) -> Self {
        Self {
            table: table.into(),
            columns,
            query,
            insert,
            update,
            delete,
        }
    }

    pub fn query(&self, where_clause: Option<&dyn IWhere>) -> SelectStatement {
        let refs: Vec<&str> = self.columns.iter().map(String::as_str).collect();
        let built = SelectStatement::new(SelectStatementOptions {
            where_clause,
            ..SelectStatementOptions::new(&self.table, &refs)
        });
        let text = map_sql(&self.query, built.get_text());
        match where_clause.and_then(|clause| clause.as_any().downcast_ref::<crate::Where>()) {
            Some(clause) => {
                let mut statement = SelectStatement::with_where(text, clause.clone());
                statement.table = self.table.clone();
                statement
            }
            None => match where_clause {
                Some(predicate) => SelectStatement::with_where(text, WhereText::new(predicate.get_text())),
                None => SelectStatement::text(text),
            },
        }
    }

    pub fn insert(&self, fields: &[Parameter]) -> InsertStatement {
        InsertStatement::text(map_sql(
            &self.insert,
            InsertStatement::new(&self.table, fields).get_text(),
        ))
    }

    pub fn update(&self, fields: &[Parameter], where_clause: &dyn IWhere) -> UpdateStatement {
        let built = UpdateStatement::new(&self.table, fields, Some(where_clause));
        UpdateStatement::text(
            map_sql(&self.update, built.get_text()),
            Some(Box::new(WhereText::new(where_clause.get_text()))),
        )
    }

    pub fn delete(&self, where_clause: &dyn IWhere) -> DeleteStatement {
        DeleteStatement::raw(
            map_sql(
                &self.delete,
                DeleteStatement::new(&self.table, Some(where_clause)).get_text(),
            ),
            Some(Box::new(WhereText::new(where_clause.get_text()))),
        )
    }
}

fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String {
    match mapper {
        Some(map) => Mapper::map(map.as_ref(), text),
        None => text,
    }
}
