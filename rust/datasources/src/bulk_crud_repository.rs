use crate::built_repository::BuiltRepository;
use crate::bulk_command::BulkCommand;
use crate::bulk_write::BulkWrite;
use crate::bulk_write_strategy::BulkWriteStrategy;
use crate::command::Command;
use crate::connection::Connection;
use crate::crud_repository::CRUDRepository;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::strip_enriched;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::unit_of_work::UnitOfWork;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::{Expression, Literal, Operator, Text, Where};
use crate::single_commands::SingleCommands;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct BulkCRUDRepository {
    crud: CRUDRepository,
    strategy: Arc<dyn BulkWriteStrategy>,
}

impl BulkCRUDRepository {
    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        Self::with_strategy(connection, options, Arc::new(SingleCommands))
    }

    pub fn with_strategy(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
        strategy: Arc<dyn BulkWriteStrategy>,
    ) -> Result<Self> {
        Ok(Self {
            crud: CRUDRepository::new(connection, options)?,
            strategy,
        })
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        Self::from_built_with(built, Arc::new(SingleCommands))
    }

    pub fn from_built_with(built: BuiltRepository, strategy: Arc<dyn BulkWriteStrategy>) -> Self {
        Self {
            crud: CRUDRepository::from_built(built),
            strategy,
        }
    }

    pub fn memory(items: Vec<Row>) -> Self {
        Self::memory_with(items, None)
    }

    pub fn memory_with(items: Vec<Row>, write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> Self {
        Self {
            crud: CRUDRepository::memory_with(items, write),
            strategy: Arc::new(SingleCommands),
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

    pub async fn create_bulk(&self, items: &mut [Row]) -> Result<Response> {
        if !self.crud.use_transaction() {
            return self.strategy.create(items, self).await;
        }
        let connection = Arc::clone(&self.crud.readonly.connection);
        UnitOfWork::new(connection)
            .in_transaction(self.strategy.create(items, self))
            .await
    }

    pub async fn update_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        if !self.crud.use_transaction() {
            return self.strategy.update(items, by_where, self).await;
        }
        let connection = Arc::clone(&self.crud.readonly.connection);
        UnitOfWork::new(connection)
            .in_transaction(self.strategy.update(items, by_where, self))
            .await
    }

    pub async fn delete_bulk(
        &self,
        items: &[Row],
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        if !self.crud.use_transaction() {
            return self.strategy.delete(items, by_where, self).await;
        }
        let connection = Arc::clone(&self.crud.readonly.connection);
        UnitOfWork::new(connection)
            .in_transaction(self.strategy.delete(items, by_where, self))
            .await
    }
}

#[async_trait]
impl BulkWrite for BulkCRUDRepository {
    fn command(&self) -> Arc<dyn Command> {
        self.crud.command()
    }

    async fn create_one(&self, item: &mut Row) -> Result<Value> {
        let created = self.crud.create_core(item).await?;
        Ok(self.crud.id_value(&created))
    }

    async fn update_one(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.update_core(item, by_where).await
    }

    async fn delete_one(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.crud.delete_core(item, by_where).await
    }

    fn inserts(&self, items: &[Row]) -> Result<Vec<BulkCommand>> {
        items
            .iter()
            .map(|item| Ok(command(insert_command(&self.crud, &stored(&self.crud, item))?)))
            .collect()
    }

    fn updates(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Vec<BulkCommand>> {
        items
            .iter()
            .map(|item| Ok(command(update_command(&self.crud, &stored(&self.crud, item), by_where)?)))
            .collect()
    }

    fn deletes(&self, items: &[Row], by_where: Option<&dyn IWhere>) -> Result<Vec<BulkCommand>> {
        items
            .iter()
            .map(|item| Ok(command(delete_command(&self.crud, &stored(&self.crud, item), by_where)?)))
            .collect()
    }

    fn rows(&self, items: &[Row]) -> Result<Vec<Row>> {
        items.iter().map(|item| stored_row(&self.crud, item)).collect()
    }
}

fn command((text, parameters): (String, Vec<Parameter>)) -> BulkCommand {
    BulkCommand { text, parameters }
}

fn stored(repo: &CRUDRepository, item: &Row) -> Row {
    if repo.readonly.enriched() {
        let entries = repo.readonly.enrichments.lock().expect("enrichments");
        strip_enriched(item, &entries)
    } else {
        item.clone()
    }
}

fn insert_command(repo: &CRUDRepository, item: &Row) -> Result<(String, Vec<Parameter>)> {
    let record = repo.readonly.reverse_convert(item)?;
    let fields = params(repo, &record, false)?;
    let statement = repo.readonly.statements.insert(&fields);
    Ok((statement.get_text(), fields))
}

fn update_command(
    repo: &CRUDRepository,
    item: &Row,
    by_where: Option<&dyn IWhere>,
) -> Result<(String, Vec<Parameter>)> {
    let record = repo.readonly.reverse_convert(item)?;
    let fields = params(repo, &record, false)?;
    if fields.is_empty() {
        return Err(DatasourceError::message(
            "Update requires at least one non-Id field",
        ));
    }
    let (owned, raw) = predicate(repo, &record, by_where)?;
    let clause: &dyn IWhere = match (owned.as_ref(), raw) {
        (Some(clause), _) => clause,
        (None, Some(clause)) => clause,
        _ => unreachable!(),
    };
    let statement = repo.readonly.statements.update(&fields, clause);
    let parameters = crate::UpdateStatement::parameter_list(&fields, Some(clause));
    Ok((statement.get_text(), parameters))
}

fn delete_command(
    repo: &CRUDRepository,
    item: &Row,
    by_where: Option<&dyn IWhere>,
) -> Result<(String, Vec<Parameter>)> {
    let record = repo.readonly.reverse_convert(item)?;
    let (owned, raw) = predicate(repo, &record, by_where)?;
    let clause: &dyn IWhere = match (owned.as_ref(), raw) {
        (Some(clause), _) => clause,
        (None, Some(clause)) => clause,
        _ => unreachable!(),
    };
    let statement = repo.readonly.statements.delete(clause);
    let parameters = clause.parameters();
    let parameters = if parameters.is_empty() && statement.get_text().contains('$') {
        params(repo, &record, true)?
    } else {
        parameters
    };
    Ok((statement.get_text(), parameters))
}

fn stored_row(repo: &CRUDRepository, item: &Row) -> Result<Row> {
    let record = repo.readonly.reverse_convert(&stored(repo, item))?;
    let mut row = Row::new();
    for name in repo.readonly.statements.columns() {
        let value = record
            .iter()
            .find(|(key, _)| repo.readonly.column(key) == *name)
            .map(|(_, value)| value.clone())
            .unwrap_or(Value::Null);
        row.insert(name.clone(), value);
    }
    Ok(row)
}

fn params(repo: &CRUDRepository, record: &Row, include_ids: bool) -> Result<Vec<Parameter>> {
    let ids = repo.id_names();
    let mut pairs: Vec<_> = record
        .iter()
        .filter(|(key, value)| (include_ids || !ids.iter().any(|id| id == *key)) && !matches!(value, Value::Null))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
        .into_iter()
        .map(|(name, value)| {
            Ok(Parameter::new(
                repo.readonly.column(&name),
                repo.readonly.cell(&name, &value, true)?,
            ))
        })
        .collect()
}

fn predicate<'a>(
    repo: &CRUDRepository,
    record: &Row,
    by_where: Option<&'a dyn IWhere>,
) -> Result<(Option<Where>, Option<&'a dyn IWhere>)> {
    match by_where {
        Some(clause) => match clause.as_any().downcast_ref::<Where>() {
            Some(typed) => Ok((Some(typed.map(|name| repo.readonly.column(name), |name, cell| repo.readonly.cell(name, cell, false))?), None)),
            None => Ok((None, Some(clause))),
        },
        None => {
            let clause = id_where(repo, record)?;
            Ok((Some(clause.map(|name| repo.readonly.column(name), |name, cell| repo.readonly.cell(name, cell, true))?), None))
        }
    }
}

fn id_where(repo: &CRUDRepository, record: &Row) -> Result<Where> {
    let children = repo
        .id_names()
        .into_iter()
        .map(|col| {
            let value = record.get(&col).ok_or_else(|| {
                DatasourceError::message(format!("Missing Id value \"{col}\""))
            })?;
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
    ) -> Result<Option<Row>> {
        self.crud.read_single(query, parameters).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[crate::Parameter]>,
    ) -> Result<Vec<Row>> {
        self.crud.read(query, parameters).await
    }
}

#[async_trait]
impl ICRUDRepository for BulkCRUDRepository {
    fn id_names(&self) -> Vec<String> {
        self.crud.id_names()
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
