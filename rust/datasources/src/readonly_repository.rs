use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_options::create_repository_options;
use crate::i_repository::IRepository;
use crate::mappers::Mapper;
use crate::parameter::Parameter;
use crate::query::Query;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::statement::Statement;
use crate::table_repository_builder::RepositoryBuilder;
use crate::table_repository_statements::RepositoryStatements;
use crate::text_query::QueryStatement;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ReadonlyRepository {
    pub(crate) connection: Arc<dyn Connection>,
    pub(crate) middlewares: Vec<Arc<dyn CommandMiddleware>>,
    pub(crate) row_converter: RowTypeConverter,
    pub(crate) rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub(crate) statements: RepositoryStatements,
    pub(crate) entity_type: TypeId,
}

impl ReadonlyRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_options::<T>();
        options.row_converter = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(connection, options.read)?;
        repo.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
    ) -> Result<Self> {
        Ok(Self::from_built(RepositoryBuilder::parts(
            connection, options,
        )?))
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        Self {
            connection: built.connection,
            middlewares: Vec::new(),
            row_converter: built.row_converter,
            rows_mapper: built.rows_mapper,
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

    pub(crate) fn query_of(&self, query: Option<&dyn Query>) -> (QueryStatement, Vec<Parameter>) {
        if let Some(query) = query {
            let params = query
                .where_clause()
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| query.parameters());
            let statement = match query.where_clause() {
                Some(predicate) => QueryStatement::with_where(
                    query.get_text(),
                    crate::text_where::TextWhere::new(predicate.get_text()),
                ),
                None => QueryStatement::text(query.get_text()),
            };
            (statement, params)
        } else {
            let statement = self.statements.query(None);
            let params = statement.parameters();
            (statement, params)
        }
    }

    pub(crate) async fn fetch_one(
        &self,
        statement: &dyn crate::statement::Statement,
        params: Vec<Parameter>,
    ) -> Result<Option<Row>> {
        let reader = self
            .command()
            .execute_reader_statement(statement, Some(&params))
            .await?;
        if !reader.next().await? {
            return Ok(None);
        }
        reader
            .current()
            .map(|row| {
                self.row_converter
                    .convert(&row)
                    .map(|converted| Mapper::map(&self.rows_mapper, converted))
            })
            .transpose()
    }

    pub(crate) async fn fetch_all(
        &self,
        statement: &dyn crate::statement::Statement,
        params: Vec<Parameter>,
    ) -> Result<Vec<Row>> {
        let reader = self
            .command()
            .execute_reader_statement(statement, Some(&params))
            .await?;
        let mut items = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                items.push(Mapper::map(
                    &self.rows_mapper,
                    self.row_converter.convert(&row)?,
                ));
            }
        }
        Ok(items)
    }

    pub(crate) fn reverse_convert(&self, item: &Row) -> Result<Row> {
        self.row_converter
            .reverse_convert(&Mapper::reverse_map(&self.rows_mapper, item.clone()))
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
        let (statement, params) = self.query_of(query);
        self.fetch_one(&statement, params).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        let (statement, params) = self.query_of(query);
        self.fetch_all(&statement, params).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        let statement = self.statements.query(Some(where_clause));
        self.fetch_one(&statement, where_clause.parameters()).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        let statement = self.statements.query(Some(where_clause));
        self.fetch_all(&statement, where_clause.parameters()).await
    }
}
