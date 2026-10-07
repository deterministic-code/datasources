use crate::bulk_command::BulkCommand;
use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::crud_repository::{open_for_bulk, CRUDRepository};
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::crud_repository_statements::CRUDRepositoryStatements;
use crate::datasource_error::DatasourceError;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::i_unit_of_work::IUnitOfWork;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::readonly_repository::ReadonlyRepository;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::single_command_i_bulk_write_strategy::SingleCommandIBulkWriteStrategy;
use crate::unit_of_work_transaction::UnitOfWorkTransaction;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::{DefaultStatementTextRenderer, Expression, Literal, Operator, Where};
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct BulkCRUDRepository {
    crud: CRUDRepository,
    crud_statements: CRUDRepositoryStatements,
    strategy: Arc<dyn BulkWriteStrategy>,
    use_transaction: bool,
}

impl BulkCRUDRepository {
    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        Self::with_strategy(
            connection,
            options,
            Arc::new(SingleCommandIBulkWriteStrategy),
        )
    }

    pub fn with_strategy(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
        strategy: Arc<dyn BulkWriteStrategy>,
    ) -> Result<Self> {
        let use_transaction = options
            .write
            .as_ref()
            .is_some_and(|write| write.use_transaction());
        let (crud, crud_statements) = open_for_bulk(connection, options)?;
        Ok(Self {
            crud,
            crud_statements,
            strategy,
            use_transaction,
        })
    }

    pub fn memory(items: Vec<Row>) -> Self {
        Self::memory_with(items, None)
    }

    pub fn memory_with(items: Vec<Row>, write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> Self {
        let use_transaction = write.as_ref().is_some_and(|write| write.use_transaction());
        Self {
            crud: CRUDRepository::memory_with(items, write),
            crud_statements: CRUDRepositoryStatements::empty(),
            strategy: Arc::new(SingleCommandIBulkWriteStrategy),
            use_transaction,
        }
    }

    pub fn add_enriched(
        &mut self,
        enrichment: crate::enrichment::Enrichment,
        via: impl EnrichmentVia,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
    ) -> &mut Self {
        self.crud.add_enriched(enrichment, via, strategy);
        self
    }

    async fn in_transaction<T, F, Fut>(
        transactional: bool,
        connection: Arc<dyn Connection>,
        write: F,
    ) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        if !transactional {
            return write().await;
        }
        let mut work = UnitOfWorkTransaction::new(connection);
        work.begin().await?;
        let result = write().await;
        match &result {
            Ok(_) => {
                if let Err(error) = work.commit().await {
                    work.abort().await?;
                    return Err(error);
                }
            }
            Err(_) => {
                work.abort().await?;
            }
        }
        result
    }

    pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Response> {
        Self::in_transaction(
            self.use_transaction,
            Arc::clone(&stored_repo(&self.crud).connection),
            || self.strategy.create(items, self),
        )
        .await
    }

    pub async fn update_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        Self::in_transaction(
            self.use_transaction,
            Arc::clone(&stored_repo(&self.crud).connection),
            || self.strategy.update(items, by_where, self),
        )
        .await
    }

    pub async fn delete_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        Self::in_transaction(
            self.use_transaction,
            Arc::clone(&stored_repo(&self.crud).connection),
            || self.strategy.delete(items, by_where, self),
        )
        .await
    }

    pub fn command(&self) -> Arc<dyn Command> {
        self.crud.command()
    }

    pub async fn create_one(&self, item: &mut Row) -> Result<Value> {
        let created = self.crud.create(item).await?;
        Ok(id_of(&self.crud, &created))
    }

    pub async fn update_one(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.update(item, by_where).await
    }

    pub async fn delete_one(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.delete(item, by_where).await
    }

    pub fn inserts(&self, items: &[Row]) -> Result<Vec<BulkCommand>> {
        items
            .iter()
            .map(|item| {
                Ok(command(insert_command(
                    &self.crud,
                    &self.crud_statements,
                    item,
                )?))
            })
            .collect()
    }

    pub fn updates(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Vec<BulkCommand>> {
        items
            .iter()
            .map(|item| {
                Ok(command(update_command(
                    &self.crud,
                    &self.crud_statements,
                    item,
                    by_where,
                )?))
            })
            .collect()
    }

    pub fn deletes(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Vec<BulkCommand>> {
        items
            .iter()
            .map(|item| {
                Ok(command(delete_command(
                    &self.crud,
                    &self.crud_statements,
                    item,
                    by_where,
                )?))
            })
            .collect()
    }

    pub fn rows(&self, items: &[Row]) -> Result<Vec<Row>> {
        items
            .iter()
            .map(|item| stored_row(&self.crud, &self.crud_statements, item))
            .collect()
    }
}

fn command((text, parameters): (String, Vec<Parameter>)) -> BulkCommand {
    BulkCommand { text, parameters }
}

fn stored_repo(repo: &CRUDRepository) -> &ReadonlyRepository {
    repo
}

fn id_of(repo: &CRUDRepository, item: &Row) -> Value {
    let ids = repo.id_names();
    if ids.len() != 1 {
        return Value::Null;
    }
    item.get(&ids[0]).cloned().unwrap_or(Value::Null)
}

fn insert_command(
    repo: &CRUDRepository,
    statements: &CRUDRepositoryStatements,
    item: &Row,
) -> Result<(String, Vec<Parameter>)> {
    let readonly = stored_repo(repo);
    let record = readonly.reverse_convert(item)?;
    let fields = insert_params(repo, &record)?;
    let statement = statements.insert(&fields);
    Ok((DefaultStatementTextRenderer::instance().render(&statement), statement.fields().to_vec()))
}

fn update_command(
    repo: &CRUDRepository,
    statements: &CRUDRepositoryStatements,
    item: &Row,
    by_where: Option<&dyn IWhere>,
) -> Result<(String, Vec<Parameter>)> {
    let readonly = stored_repo(repo);
    let record = readonly.reverse_convert(item)?;
    let fields = params(repo, &record, false)?;
    if fields.is_empty() {
        return Err(DatasourceError::message(
            "Update requires at least one non-Id field",
        ));
    }
    let owned;
    let clause: &dyn IWhere = match by_where {
        Some(clause) => clause,
        None => {
            owned = id_where(repo, &record)?;
            &owned
        }
    };
    let applied = by_where.is_none();
    let statement = statements.update(&fields, clause, &|name, value| {
        stored_repo(repo).cell(name, value, applied)
    })?;
    Ok((DefaultStatementTextRenderer::instance().render(&statement), statement.parameters()))
}

fn delete_command(
    repo: &CRUDRepository,
    statements: &CRUDRepositoryStatements,
    item: &Row,
    by_where: Option<&dyn IWhere>,
) -> Result<(String, Vec<Parameter>)> {
    let readonly = stored_repo(repo);
    let record = readonly.reverse_convert(item)?;
    let owned;
    let clause: &dyn IWhere = match by_where {
        Some(clause) => clause,
        None => {
            owned = id_where(repo, &record)?;
            &owned
        }
    };
    let applied = by_where.is_none();
    let statement = statements.delete(clause, &|name, value| {
        stored_repo(repo).cell(name, value, applied)
    })?;
    let parameters = statement
        .where_clause()
        .map(|clause| clause.parameters())
        .unwrap_or_default();
    let rendered = DefaultStatementTextRenderer::instance().render(&statement);
    let parameters = if parameters.is_empty() && rendered.contains('$') {
        params(repo, &record, true)?
    } else {
        parameters
    };
    Ok((rendered, parameters))
}

fn stored_row(
    repo: &CRUDRepository,
    statements: &CRUDRepositoryStatements,
    item: &Row,
) -> Result<Row> {
    let readonly = stored_repo(repo);
    let record = readonly.reverse_convert(item)?;
    let mut row = Row::new();
    for name in statements.read_columns() {
        let value = record
            .iter()
            .find(|(key, _)| readonly.column(key) == *name && !repo.enrichment_into(key))
            .map(|(_, value)| value.clone())
            .unwrap_or(Value::Null);
        row.insert(name.clone(), value);
    }
    Ok(row)
}

fn insert_params(repo: &CRUDRepository, record: &Row) -> Result<Vec<Parameter>> {
    let ids = repo.id_names();
    fields(repo, record, |key, value| {
        !ids.iter().any(|id| id == key) || assigned(value)
    })
}

fn params(repo: &CRUDRepository, record: &Row, include_ids: bool) -> Result<Vec<Parameter>> {
    let ids = repo.id_names();
    fields(repo, record, |key, _| include_ids || !ids.iter().any(|id| id == key))
}

fn assigned(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Text(text) => !text.is_empty(),
        Value::Int(0) => false,
        _ => true,
    }
}

fn fields(
    repo: &CRUDRepository,
    record: &Row,
    include: impl Fn(&str, &Value) -> bool,
) -> Result<Vec<Parameter>> {
    let mut pairs: Vec<_> = record
        .iter()
        .filter(|(key, value)| {
            include(key, value)
                && !repo.enrichment_into(key)
                && !matches!(value, Value::Null)
        })
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
        .into_iter()
        .map(|(name, value)| {
            Ok(Parameter::new(
                name.clone(),
                stored_repo(repo).cell(&name, &value, true)?,
            ))
        })
        .collect()
}

fn id_where(repo: &CRUDRepository, record: &Row) -> Result<Where> {
    let children = repo
        .id_names()
        .into_iter()
        .map(|col| {
            let value = record
                .get(&col)
                .ok_or_else(|| DatasourceError::message(format!("Missing Id value \"{col}\"")))?;
            Ok(Expression::from(Operator::equals(col, value.clone())))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Where::new(and_all(children)))
}

fn and_all(parts: Vec<Expression>) -> Expression {
    parts
        .into_iter()
        .reduce(|left, right| Operator::and(left, right).into())
        .unwrap_or_else(|| {
            Operator::Equals {
                left: Box::new(Expression::Literal(Literal::new(1_i64))),
                right: Box::new(Expression::Literal(Literal::new(1_i64))),
            }
            .into()
        })
}

impl Deref for BulkCRUDRepository {
    type Target = CRUDRepository;

    fn deref(&self) -> &Self::Target {
        &self.crud
    }
}

impl DerefMut for BulkCRUDRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.crud
    }
}

impl IRepository for BulkCRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.crud.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for BulkCRUDRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
        options: Option<&crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        self.crud.read_single(query, parameters, options).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
        options: Option<&crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<Row>> {
        self.crud.read(query, parameters, options).await
    }
}

#[async_trait]
impl ICRUDRepository for BulkCRUDRepository {
    fn id_names(&self) -> Vec<String> {
        self.crud.id_names()
    }

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.crud.add_middleware(middleware);
    }

    async fn create(&self, item: &mut Row) -> Result<Row> {
        self.crud.create(item).await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.update(item, by_where).await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.delete(item, by_where).await
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        self.crud.read_single_by_id(id).await
    }
}
