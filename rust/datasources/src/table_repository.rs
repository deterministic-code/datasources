use crate::builders::WhereBuilder;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::create_repository_builder_options::create_repository_builder_options;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::datasource_error::DatasourceError;
use crate::parameter::Parameter;
use crate::query::Query;
use crate::row::Row;
use crate::repository::{ReadonlyRepository, Repository};
use crate::response::Response;
use crate::result::Result;
use crate::built_repository::BuiltRepository;
use crate::table_repository_builder::TableRepositoryBuilder;
use crate::table_repository_builder_options::TableRepositoryBuilderOptions;
use crate::table_repository_statements::TableRepositoryStatements;
use crate::where_trait::Where;
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

pub struct TableRepository {
    connection: Arc<dyn Connection>,
    middlewares: Vec<Arc<dyn CommandMiddleware>>,
    row_converter: RowTypeConverter,
    statements: TableRepositoryStatements,
    id: Vec<String>,
    ids: HashSet<String>,
    assign_identity_on_add: bool,
}

impl TableRepository {
    pub fn from_type<T: ConvertSchema>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_repository_builder_options::<T>(connection.clone());
        options.table = std::any::type_name::<T>()
            .rsplit("::")
            .next()
            .unwrap_or_default()
            .into();
        options.mapping = Some(RowTypeConverter::of::<T>());
        Self::new(options)
    }

    pub fn new(options: TableRepositoryBuilderOptions) -> Result<Self> {
        TableRepositoryBuilder::build(options)
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        Self::from_parts(
            built.connection,
            built.row_converter,
            built.statements,
            built.ids,
            built.assign_identity_on_add,
        )
    }

    pub fn from_parts(
        connection: Arc<dyn Connection>,
        row_converter: RowTypeConverter,
        statements: TableRepositoryStatements,
        ids: Vec<String>,
        assign_identity_on_add: bool,
    ) -> Self {
        Self {
            connection,
            middlewares: Vec::new(),
            row_converter,
            statements,
            ids: ids.iter().cloned().collect(),
            id: ids,
            assign_identity_on_add,
        }
    }

    pub fn use_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.middlewares.push(middleware);
    }

    fn command(&self) -> Arc<dyn Command> {
        let command = self.connection.create_command();
        for middleware in &self.middlewares {
            command.use_middleware(middleware.clone());
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

    async fn read_single(&self, text: String, params: Vec<Parameter>) -> Result<Option<Row>> {
        let reader = self
            .command()
            .execute_reader(&text, Some(&params))
            .await?;
        if !reader.next().await? {
            return Ok(None);
        }
        reader
            .current()
            .map(|row| self.row_converter.convert(&row))
            .transpose()
    }

    async fn read_all(&self, text: String, params: Vec<Parameter>) -> Result<Vec<Row>> {
        let reader = self
            .command()
            .execute_reader(&text, Some(&params))
            .await?;
        let mut items = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                items.push(self.row_converter.convert(&row)?);
            }
        }
        Ok(items)
    }

    fn sql(&self, row: &Row) -> Result<Row> {
        self.row_converter.reverse_convert(row)
    }
}

#[async_trait]
impl ReadonlyRepository for TableRepository {
    async fn get_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        let (text, params) = self.query_parts(query);
        self.read_single(text, params).await
    }

    async fn get(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        let (text, params) = self.query_parts(query);
        self.read_all(text, params).await
    }

    async fn get_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        let (text, params) = self.statements.query(Some(where_clause));
        self.read_single(text, params).await
    }

    async fn get_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        let (text, params) = self.statements.query(Some(where_clause));
        self.read_all(text, params).await
    }
}

#[async_trait]
impl Repository for TableRepository {
    async fn add(&self, obj: &Row) -> Result<Response> {
        let record = self.sql(obj)?;
        let fields = self.params(&record, !self.assign_identity_on_add);
        let text = self.statements.insert(&fields);
        self.command()
            .execute(&text, Some(&fields))
            .await
    }

    async fn update(&self, obj: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.sql(obj)?;
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
        self.command()
            .execute(&text, Some(&parameters))
            .await
    }

    async fn delete(&self, obj: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.sql(obj)?;
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
        self.command()
            .execute(&text, Some(&parameters))
            .await
    }
}
