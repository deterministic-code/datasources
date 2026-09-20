use crate::builders::WhereBuilder;
use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_builder_options::create_repository_builder_options;
use crate::datasource_error::DatasourceError;
use crate::mappers::{Mapper, StatementMapper};
use crate::parameter::Parameter;
use crate::query::Query;
use crate::repository::{CRUDRepository, ReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::table_repository_builder::RepositoryBuilder;
use crate::table_repository_builder_options::RepositoryBuilderOptions;
use crate::table_repository_statements::RepositoryStatements;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::collections::HashSet;
use std::sync::Arc;

pub struct Repository {
    connection: Arc<dyn Connection>,
    middlewares: Vec<Arc<dyn CommandMiddleware>>,
    row_converter: RowTypeConverter,
    statement_mapper: Arc<StatementMapper>,
    statements: RepositoryStatements,
    id: Vec<String>,
    ids: HashSet<String>,
    assign_identity_on_create: bool,
    entity_type: TypeId,
}

impl Repository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_builder_options::<T>(connection.clone());
        options.mapping = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(options)?;
        repo.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(options: RepositoryBuilderOptions) -> Result<Self> {
        RepositoryBuilder::build(options)
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        Self::from_parts(
            built.connection,
            built.row_converter,
            built.statement_mapper,
            built.statements,
            built.ids,
            built.assign_identity_on_create,
        )
    }

    pub fn from_parts(
        connection: Arc<dyn Connection>,
        row_converter: RowTypeConverter,
        statement_mapper: Arc<StatementMapper>,
        statements: RepositoryStatements,
        ids: Vec<String>,
        assign_identity_on_create: bool,
    ) -> Self {
        Self {
            connection,
            middlewares: Vec::new(),
            row_converter,
            statement_mapper,
            statements,
            ids: ids.iter().cloned().collect(),
            id: ids,
            assign_identity_on_create,
            entity_type: TypeId::of::<Row>(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.middlewares.push(middleware);
    }

    fn command(&self) -> Arc<dyn Command> {
        let command = self.connection.create_command();
        for middleware in &self.middlewares {
            command.add_middleware(middleware.clone());
        }
        command
    }

    fn params(&self, record: &Row, include_id: bool) -> Vec<Parameter> {
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

    fn id_where(&self, record: &Row) -> Result<WhereBuilder> {
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

    async fn fetch_one(&self, text: String, params: Vec<Parameter>) -> Result<Option<Row>> {
        let reader = self.command().execute_reader(&text, Some(&params)).await?;
        if !reader.next().await? {
            return Ok(None);
        }
        reader
            .current()
            .map(|row| {
                self.row_converter
                    .convert(&row)
                    .map(|converted| Mapper::map(&self.statement_mapper.rows, converted))
            })
            .transpose()
    }

    async fn fetch_all(&self, text: String, params: Vec<Parameter>) -> Result<Vec<Row>> {
        let reader = self.command().execute_reader(&text, Some(&params)).await?;
        let mut items = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                items.push(Mapper::map(
                    &self.statement_mapper.rows,
                    self.row_converter.convert(&row)?,
                ));
            }
        }
        Ok(items)
    }

    fn reverse_convert(&self, item: &Row) -> Result<Row> {
        self.row_converter.reverse_convert(&Mapper::reverse_map(
            &self.statement_mapper.rows,
            item.clone(),
        ))
    }
}

#[async_trait]
impl ReadonlyRepository for Repository {
    fn entity_type(&self) -> TypeId {
        self.entity_type
    }

    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        let (text, params) = self.query_parts(query);
        self.fetch_one(text, params).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        let (text, params) = self.query_parts(query);
        self.fetch_all(text, params).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        let (text, params) = self.statements.query(Some(where_clause));
        self.fetch_one(text, params).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        let (text, params) = self.statements.query(Some(where_clause));
        self.fetch_all(text, params).await
    }
}

#[async_trait]
impl CRUDRepository for Repository {
    async fn create(&self, item: &Row) -> Result<Response> {
        let record = self.reverse_convert(item)?;
        let fields = self.params(&record, !self.assign_identity_on_create);
        let text = self.statements.insert(&fields);
        self.command().execute(&text, Some(&fields)).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.reverse_convert(item)?;
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
        self.command().execute(&text, Some(&parameters)).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.reverse_convert(item)?;
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
        self.command().execute(&text, Some(&parameters)).await
    }
}
