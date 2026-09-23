use crate::mappers::Mapper;
use crate::mappers::TransformMapper;
use crate::parameter::Parameter;
use crate::statements::{DeleteStatement, InsertStatement, Text, UpdateStatement};
use crate::text_where::TextWhere;
use crate::where_trait::Where;
use crate::QueryStatement;
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

    pub fn query(&self, where_clause: Option<&dyn Where>) -> QueryStatement {
        let refs: Vec<&str> = self.columns.iter().map(String::as_str).collect();
        let built = QueryStatement::new(&self.table, &refs, where_clause, &[]);
        let text = map_sql(&self.query, built.get_text());
        match where_clause {
            Some(predicate) => {
                QueryStatement::with_where(text, TextWhere::new(predicate.get_text()))
            }
            None => QueryStatement::text(text),
        }
    }

    pub fn insert(&self, fields: &[Parameter]) -> InsertStatement {
        InsertStatement::text(map_sql(
            &self.insert,
            InsertStatement::new(&self.table, fields).get_text(),
        ))
    }

    pub fn update(&self, fields: &[Parameter], where_clause: &dyn Where) -> UpdateStatement {
        UpdateStatement::text(
            map_sql(
                &self.update,
                UpdateStatement::new(&self.table, fields, Some(where_clause)).get_text(),
            ),
            Some(Box::new(TextWhere::new(where_clause.get_text()))),
        )
    }

    pub fn delete(&self, where_clause: &dyn Where) -> DeleteStatement {
        DeleteStatement::raw(
            map_sql(
                &self.delete,
                DeleteStatement::new(&self.table, Some(where_clause)).get_text(),
            ),
            Some(Box::new(TextWhere::new(where_clause.get_text()))),
        )
    }
}

fn map_sql(mapper: &Option<Arc<TransformMapper<String, String>>>, text: String) -> String {
    match mapper {
        Some(map) => Mapper::map(map.as_ref(), text),
        None => text,
    }
}
