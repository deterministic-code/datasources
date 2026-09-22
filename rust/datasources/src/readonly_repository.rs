use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_builder_options::create_repository_builder_options;
use crate::i_repository::IRepository;
use crate::mappers::{Mapper, StatementMapper};
use crate::parameter::Parameter;
use crate::query::Query;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::table_repository_builder::RepositoryBuilder;
use crate::table_repository_builder_options::RepositoryBuilderOptions;
use crate::table_repository_statements::RepositoryStatements;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ReadonlyRepository {
    pub(crate) connection: Arc<dyn Connection>,
    pub(crate) middlewares: Vec<Arc<dyn CommandMiddleware>>,
    pub(crate) row_converter: RowTypeConverter,
    pub(crate) statement_mapper: Arc<StatementMapper>,
    pub(crate) statements: RepositoryStatements,
    pub(crate) entity_type: TypeId,
}

impl ReadonlyRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_builder_options::<T>(connection.clone());
        options.mapping = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(options)?;
        repo.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(options: RepositoryBuilderOptions) -> Result<Self> {
        Ok(Self::from_built(RepositoryBuilder::parts(options)?))
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        Self {
            connection: built.connection,
            middlewares: Vec::new(),
            row_converter: built.row_converter,
            statement_mapper: built.statement_mapper,
            statements: built.statements,
            entity_type: TypeId::of::<Row>(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.middlewares.push(middleware);
    }

    pub(crate) fn command(&self) -> Arc<dyn Command> {
        let command = self.connection.create_command();
        for middleware in &self.middlewares {
            command.add_middleware(middleware.clone());
        }
        command
    }

    pub(crate) fn query_parts(&self, query: Option<&dyn Query>) -> (String, Vec<Parameter>) {
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

    pub(crate) async fn fetch_one(&self, text: String, params: Vec<Parameter>) -> Result<Option<Row>> {
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

    pub(crate) async fn fetch_all(&self, text: String, params: Vec<Parameter>) -> Result<Vec<Row>> {
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

    pub(crate) fn reverse_convert(&self, item: &Row) -> Result<Row> {
        self.row_converter.reverse_convert(&Mapper::reverse_map(
            &self.statement_mapper.rows,
            item.clone(),
        ))
    }
}

impl IRepository for ReadonlyRepository {
    fn entity_type(&self) -> TypeId {
        self.entity_type
    }
}

#[async_trait]
impl IReadonlyRepository for ReadonlyRepository {
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
