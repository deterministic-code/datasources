use crate::IWhere;
use crate::mappers::IdentityMapper;
use crate::parameter::Parameter;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::result::Result;
use crate::statement_mappers::StatementMappers;
use crate::statements::{
    DeleteStatement, InsertStatement, SelectInsertedIdStatement, UpdateStatement,
};
use crate::value::Value;
use std::ops::Deref;
use std::sync::Arc;

pub struct CRUDRepositoryStatements {
    read: ReadonlyRepositoryStatements,
}

impl CRUDRepositoryStatements {
    pub fn empty() -> Self {
        Self::new(
            "",
            Vec::new(),
            StatementMappers::new(
                Arc::new(IdentityMapper::<String>::new()),
                Arc::new(IdentityMapper::<String>::new()),
            ),
        )
    }

    pub fn new(
        table: impl Into<String>,
        columns: Vec<String>,
        mappings: StatementMappers,
    ) -> Self {
        Self {
            read: ReadonlyRepositoryStatements::new(table, columns, mappings),
        }
    }

    pub fn insert(&self, fields: &[Parameter]) -> InsertStatement {
        self.read
            .mappings()
            .apply_insert(&InsertStatement::new(self.table(), fields))
    }

    pub fn update(
        &self,
        fields: &[Parameter],
        where_clause: &dyn IWhere,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<UpdateStatement> {
        self.read.mappings().apply_update(
            &UpdateStatement::new(self.table(), fields, Some(where_clause)),
            cell,
        )
    }

    pub fn select_inserted_id(&self, column: &str) -> SelectInsertedIdStatement {
        self.read
            .mappings()
            .apply_inserted(&SelectInsertedIdStatement::new(self.table(), column))
    }

    pub fn delete(
        &self,
        where_clause: &dyn IWhere,
        cell: &dyn Fn(&str, &Value) -> Result<Value>,
    ) -> Result<DeleteStatement> {
        self.read.mappings().apply_delete(
            &DeleteStatement::new(self.table(), Some(where_clause)),
            cell,
        )
    }
}

impl Deref for CRUDRepositoryStatements {
    type Target = ReadonlyRepositoryStatements;

    fn deref(&self) -> &Self::Target {
        &self.read
    }
}
