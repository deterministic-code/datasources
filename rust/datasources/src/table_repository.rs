use crate::builders::WhereBuilder;
use crate::connection::Connection;
use crate::create_repository_builder_options::create_repository_builder_options;
use crate::converters::{mapping, ConvertSchema, TypeMapping};
use crate::datasource_error::DatasourceError;
use crate::mappers::DatasourceMapper;
use crate::parameter::Parameter;
use crate::query::Query;
use crate::record::Record;
use crate::repository::{ReadonlyRepository, Repository};
use crate::response::Response;
use crate::result::Result;
use crate::table_repository_builder::{
    TableRepositoryBuilder, TableRepositoryCommands, TableRepositoryStatements,
};
use crate::table_repository_builder_options::TableRepositoryBuilderOptions;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

pub struct TableRepository {
    commands: TableRepositoryCommands,
    converters: TypeMapping,
    pub mapping: DatasourceMapper,
    statements: TableRepositoryStatements,
    id: Vec<String>,
    ids: HashSet<String>,
    assign_identity_on_add: bool,
}

impl TableRepository {
    pub fn from_type<T: ConvertSchema>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_builder_options::<T>(connection.clone());
        options.mapping = Some(mapping::<T>(connection.provider(), "rust")?);
        Self::new(options)
    }

    pub fn new(options: TableRepositoryBuilderOptions) -> Result<Self> {
        TableRepositoryBuilder::build(options)
    }

    pub fn from_parts(
        commands: TableRepositoryCommands,
        converters: TypeMapping,
        mapping: DatasourceMapper,
        statements: TableRepositoryStatements,
        ids: Vec<String>,
        assign_identity_on_add: bool,
    ) -> Self {
        Self {
            commands,
            converters,
            mapping,
            statements,
            ids: ids.iter().cloned().collect(),
            id: ids,
            assign_identity_on_add,
        }
    }

    fn params(&self, record: &Record, include_id: bool) -> Vec<Parameter> {
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, _)| include_id || !self.ids.contains(*key))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        pairs
            .into_iter()
            .map(|(name, value)| Parameter::new(name, value))
            .collect()
    }

    fn id_where(&self, record: &Record) -> Result<WhereBuilder> {
        let children = self
            .id
            .iter()
            .map(|col| {
                let value = record.get(col).ok_or_else(|| {
                    DatasourceError::message(format!("Missing Id value \"{col}\""))
                })?;
                Ok(WhereBuilder::equals(col.clone(), value.clone()))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(WhereBuilder::and(children))
    }

    fn query_parts(&self, query: Option<&dyn Query>) -> (String, Vec<Parameter>) {
        if let Some(query) = query {
            let params = query
                .where_clause()
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| query.parameters());
            (query.get_text(), params)
        } else {
            self.statements.query(None)
        }
    }
}

#[async_trait]
impl ReadonlyRepository for TableRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Record>> {
        let (text, params) = self.query_parts(query);
        let reader = self
            .commands
            .get
            .execute_reader(Some(&text), Some(&params))
            .await?;
        if !reader.next().await? {
            return Ok(None);
        }
        reader
            .current()
            .map(|row| self.converters.to(&row))
            .transpose()
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Record>> {
        let (text, params) = self.query_parts(query);
        let reader = self
            .commands
            .get
            .execute_reader(Some(&text), Some(&params))
            .await?;
        let mut items = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                items.push(self.converters.to(&row)?);
            }
        }
        Ok(items)
    }
}

#[async_trait]
impl Repository for TableRepository {
    async fn add(&self, obj: &Record) -> Result<Response> {
        let record = self.converters.from(obj)?;
        let fields = self.params(&record, !self.assign_identity_on_add);
        let text = self.statements.insert(&fields);
        self.commands
            .add
            .execute(Some(&text), Some(&fields))
            .await
    }

    async fn update(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.converters.from(obj)?;
        let owned = if by_where.is_none() {
            Some(self.id_where(&record)?)
        } else {
            None
        };
        let predicate: &dyn Where = match (by_where, owned.as_ref()) {
            (Some(where_clause), _) => where_clause,
            (_, Some(where_clause)) => where_clause,
            _ => unreachable!(),
        };
        let fields = self.params(&record, false);
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let text = self.statements.update(&fields, predicate);
        let mut parameters = fields;
        parameters.extend(predicate.parameters());
        self.commands
            .update
            .execute(Some(&text), Some(&parameters))
            .await
    }

    async fn delete(&self, obj: &Record, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.converters.from(obj)?;
        let owned = if by_where.is_none() {
            Some(self.id_where(&record)?)
        } else {
            None
        };
        let predicate: &dyn Where = match (by_where, owned.as_ref()) {
            (Some(where_clause), _) => where_clause,
            (_, Some(where_clause)) => where_clause,
            _ => unreachable!(),
        };
        let text = self.statements.delete(predicate);
        let parameters = predicate.parameters();
        self.commands
            .delete
            .execute(Some(&text), Some(&parameters))
            .await
    }
}
