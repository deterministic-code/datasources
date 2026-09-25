use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_options::create_repository_options;
use crate::datasource_error::DatasourceError;
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::mappers::Mapper;
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::select_statement::SelectStatement;
use crate::statement::{Statement, StatementKind};
use crate::statements::Where;
use crate::table_repository_builder::RepositoryBuilder;
use crate::table_repository_statements::RepositoryStatements;
use crate::value::Value;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::Arc;

pub struct ReadonlyRepository {
    pub(crate) connection: Arc<dyn Connection>,
    pub(crate) middlewares: Vec<Arc<dyn CommandMiddleware>>,
    pub(crate) row_converter: RowTypeConverter,
    pub(crate) rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub(crate) column_mappings: Box<dyn Mapper<String, String> + Send + Sync>,
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
            column_mappings: built.column_mappings,
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

    pub(crate) fn query_of(
        &self,
        query: Option<&dyn Statement>,
    ) -> (SelectStatement, Vec<Parameter>) {
        if let Some(query) = query {
            let params = query
                .predicate()
                .map(|predicate| predicate.parameters())
                .unwrap_or_else(|| query.parameters());
            let statement = match query.predicate() {
                Some(predicate) => SelectStatement::with_where(
                    query.get_text(),
                    crate::where_text::WhereText::new(predicate.get_text()),
                ),
                None => SelectStatement::text(query.get_text()),
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
                    .convert(&self.renamed(row))
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
                    self.row_converter.convert(&self.renamed(row))?,
                ));
            }
        }
        Ok(items)
    }

    fn renamed(&self, row: Row) -> Row {
        row.into_iter()
            .map(|(key, value)| (Mapper::reverse_map(&self.column_mappings, key), value))
            .collect()
    }

    pub(crate) fn reverse_convert(&self, item: &Row) -> Result<Row> {
        self.row_converter
            .reverse_convert(&Mapper::reverse_map(&self.rows_mapper, item.clone()))
    }

    fn stored_query(&self, where_clause: &dyn IWhere) -> Result<(SelectStatement, Vec<Parameter>)> {
        match self.store_where(where_clause, false)? {
            Some(stored) => Ok((self.statements.query(Some(&stored)), stored.parameters())),
            None => Ok((
                self.statements.query(Some(where_clause)),
                where_clause.parameters(),
            )),
        }
    }

    pub(crate) fn store_where(
        &self,
        where_clause: &dyn IWhere,
        fields_applied: bool,
    ) -> Result<Option<Where>> {
        match where_clause.as_any().downcast_ref::<Where>() {
            Some(clause) => Ok(Some(self.store_clause(clause, fields_applied)?)),
            None => Ok(None),
        }
    }

    pub(crate) fn store_clause(&self, clause: &Where, fields_applied: bool) -> Result<Where> {
        clause.map(
            |name| self.column(name),
            |name, cell| self.cell(name, cell, fields_applied),
        )
    }

    pub(crate) fn column(&self, property: &str) -> String {
        Mapper::map(&self.column_mappings, property.to_string())
    }

    pub(crate) fn cell(
        &self,
        property: &str,
        value: &Value,
        fields_applied: bool,
    ) -> Result<Value> {
        match self.row_converter.fields.get(property) {
            Some(field) if !fields_applied => field.reverse_convert(value),
            _ => Ok(value.clone()),
        }
    }

    fn ensure_readable(&self, query: Option<&dyn Statement>) -> Result<()> {
        match query.map(|statement| statement.kind()) {
            Some(
                StatementKind::Insert
                | StatementKind::Update
                | StatementKind::Delete
                | StatementKind::Other,
            ) => Err(DatasourceError::message(
                "Read requires a query or stored procedure",
            )),
            _ => Ok(()),
        }
    }
}

impl IRepository for ReadonlyRepository {
    fn entity_type(&self) -> TypeId {
        self.entity_type
    }
}

#[async_trait]
impl IReadonlyRepository for ReadonlyRepository {
    async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>> {
        if let Some(where_clause) = query.and_then(ReadInput::as_where) {
            let (statement, params) = self.stored_query(where_clause)?;
            return self.fetch_one(&statement, params).await;
        }
        let statement = query.and_then(ReadInput::as_statement);
        self.ensure_readable(statement)?;
        match statement {
            Some(statement) if statement.kind() == StatementKind::Procedure => {
                self.fetch_one(statement, statement.parameters()).await
            }
            statement => {
                let (statement, params) = self.query_of(statement);
                self.fetch_one(&statement, params).await
            }
        }
    }

    async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>> {
        if let Some(where_clause) = query.and_then(ReadInput::as_where) {
            let (statement, params) = self.stored_query(where_clause)?;
            return self.fetch_all(&statement, params).await;
        }
        let statement = query.and_then(ReadInput::as_statement);
        self.ensure_readable(statement)?;
        match statement {
            Some(statement) if statement.kind() == StatementKind::Procedure => {
                self.fetch_all(statement, statement.parameters()).await
            }
            statement => {
                let (statement, params) = self.query_of(statement);
                self.fetch_all(&statement, params).await
            }
        }
    }
}
