use crate::i_where::IWhere;
use crate::parameter::Parameter;
use crate::statements::{
    DeleteStatement, InsertStatement, SelectInsertedIdStatement, Text, UpdateStatement,
};
use crate::where_text::WhereText;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use std::ops::Deref;

pub struct CRUDRepositoryStatements {
    read: ReadonlyRepositoryStatements,
}

impl CRUDRepositoryStatements {
    pub fn new(table: impl Into<String>, columns: Vec<String>) -> Self {
        Self {
            read: ReadonlyRepositoryStatements::new(table, columns),
        }
    }

    pub fn insert(&self, fields: &[Parameter]) -> InsertStatement {
        InsertStatement::text(InsertStatement::new(self.table(), fields).get_text())
    }

    pub fn update(&self, fields: &[Parameter], where_clause: &dyn IWhere) -> UpdateStatement {
        let built = UpdateStatement::new(self.table(), fields, Some(where_clause));
        UpdateStatement::text(
            built.get_text(),
            Some(Box::new(WhereText::new(where_clause.get_text()))),
        )
    }

    pub fn select_inserted_id(&self, column: &str) -> SelectInsertedIdStatement {
        SelectInsertedIdStatement::new(self.table(), column)
    }

    pub fn delete(&self, where_clause: &dyn IWhere) -> DeleteStatement {
        DeleteStatement::raw(
            DeleteStatement::new(self.table(), Some(where_clause)).get_text(),
            Some(Box::new(WhereText::new(where_clause.get_text()))),
        )
    }
}

impl Deref for CRUDRepositoryStatements {
    type Target = ReadonlyRepositoryStatements;

    fn deref(&self) -> &Self::Target {
        &self.read
    }
}
