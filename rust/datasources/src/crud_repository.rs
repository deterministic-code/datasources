use crate::built_repository::BuiltRepository;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_crud_repository_options::create_crud_repository_options;
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::datasource_error::DatasourceError;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::i_unit_of_work::IUnitOfWork;
use crate::i_where::IWhere;
use crate::id_key::IdKey;
use crate::mappers::{Mapper, TransformMapper};
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::readonly_repository::ReadonlyRepository;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::statements::ReturnStatement;
use crate::stored_enrichment::StoredEnrichment;
use crate::table_repository_builder::RepositoryBuilder;
use crate::unit_of_work_transaction::UnitOfWorkTransaction;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::write_mode::WriteMode;
use crate::{Expression, Literal, Operator, Text, Where};
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CRUDRepository {
    readonly: ReadonlyRepository,
    ids: Vec<String>,
    write: Arc<dyn WriteEnrichmentStrategy>,
    return_sql: Option<Arc<TransformMapper<String, String>>>,
}

impl RepositoryBuilder {
    pub fn build(
        connection: Arc<dyn Connection>,
        options: CRUDRepositoryOptions,
    ) -> Result<CRUDRepository> {
        CRUDRepository::new(connection, options)
    }
}

impl CRUDRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_crud_repository_options::<T>();
        options.row_converter = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(connection, options)?;
        repo.readonly.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        let return_sql = options.return_.clone();
        let mut repo = Self::from_built(RepositoryBuilder::parts_crud(connection, options)?);
        repo.return_sql = return_sql;
        Ok(repo)
    }

    fn from_built(mut built: BuiltRepository) -> Self {
        let id = built.ids.clone();
        let write = built
            .write
            .take()
            .unwrap_or_else(|| Arc::new(DefaultWriteEnrichmentStrategy::default()));
        Self {
            readonly: ReadonlyRepository::from_built(built),
            ids: id,
            write,
            return_sql: None,
        }
    }

    pub fn memory(items: Vec<Row>) -> Self {
        Self::memory_with(items, None)
    }

    pub fn memory_with(items: Vec<Row>, write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> Self {
        let ids = vec!["id".into()];
        Self {
            readonly: ReadonlyRepository::memory(items),
            ids,
            write: write.unwrap_or_else(|| Arc::new(DefaultWriteEnrichmentStrategy::default())),
            return_sql: None,
        }
    }

    pub fn with_write(mut self, write: Arc<dyn WriteEnrichmentStrategy>) -> Self {
        self.write = write;
        self
    }

    pub fn add_enriched(
        &mut self,
        enrichment: crate::enrichment::Enrichment,
        via: impl EnrichmentVia,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
    ) -> &mut Self {
        let strategy = strategy.unwrap_or_else(|| Arc::new(DefaultReadonlyEnrichmentStrategy));
        let (via, persist) = via.into_repos();
        self.readonly
            .enrichments
            .lock()
            .expect("enrichments")
            .push(StoredEnrichment::direct(enrichment, via, persist, strategy));
        self
    }

    pub fn add_linked(
        &mut self,
        enrichment: crate::enrichment::Enrichment,
        via: impl EnrichmentVia,
        link: Arc<dyn ICRUDRepository>,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
    ) -> &mut Self {
        let strategy = strategy.unwrap_or_else(|| Arc::new(DefaultReadonlyEnrichmentStrategy));
        let (via, _) = via.into_repos();
        self.readonly
            .enrichments
            .lock()
            .expect("enrichments")
            .push(StoredEnrichment::linked(enrichment, via, link, strategy));
        self
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.readonly.add_middleware(middleware);
    }

    fn enriched(&self) -> bool {
        self.readonly.enriched()
    }

    fn use_transaction(&self) -> bool {
        self.write.use_transaction()
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

    async fn create_core(&self, item: &mut Row) -> Result<Row> {
        if !self.enriched() {
            return self.inserted(item).await;
        }
        if let Some(memory) = self.readonly.memory_items().cloned() {
            let mut stored = self.without_enrichment(item);
            let created = memory.insert(&mut stored);
            let id = self.id_value(&created);
            self.write_id(item, &id)?;
        } else {
            let stored = item.clone();
            self.execute_insert(&stored, item).await?;
        }
        self.write
            .persist(
                &self.readonly.tree,
                std::slice::from_mut(item),
                WriteMode::Create,
            )
            .await?;
        let id = self.id_value(item);
        if self.ids.len() != 1 || matches!(id, Value::Null) {
            return Ok(item.clone());
        }
        Ok(self
            .read_single_by_id(&id)
            .await?
            .unwrap_or_else(|| item.clone()))
    }

    async fn update_core(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        let response = self.save(item, by_where).await?;
        if self.enriched() {
            let mut parent = item.clone();
            self.write
                .persist(
                    &self.readonly.tree,
                    std::slice::from_mut(&mut parent),
                    WriteMode::Update,
                )
                .await?;
        }
        Ok(response)
    }

    async fn delete_core(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        if self.enriched() {
            let mut parent = item.clone();
            self.write
                .persist(
                    &self.readonly.tree,
                    std::slice::from_mut(&mut parent),
                    WriteMode::Delete,
                )
                .await?;
        }
        self.remove(item, by_where).await
    }

    fn params(&self, record: &Row, include_ids: bool) -> Result<Vec<Parameter>> {
        let entries = self.readonly.enrichments.lock().expect("enrichments");
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, value)| {
                (include_ids || !self.ids.contains(*key))
                    && !entries.iter().any(|entry| entry.enrichment.into == **key)
                    && !matches!(value, Value::Null)
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        drop(entries);
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        pairs
            .into_iter()
            .map(|(name, value)| {
                Ok(Parameter::new(
                    self.readonly.column(&name),
                    self.readonly.cell(&name, &value, true)?,
                ))
            })
            .collect()
    }

    fn id_where(&self, record: &Row) -> Result<Where> {
        let children = self
            .ids
            .iter()
            .map(|col| {
                let value = record.get(col).ok_or_else(|| {
                    DatasourceError::message(format!("Missing Id value \"{col}\""))
                })?;
                Ok(Expression::from(Operator::equals(
                    col.clone(),
                    value.clone(),
                )))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Where::new(and_all(children)))
    }

    async fn inserted(&self, item: &mut Row) -> Result<Row> {
        if let Some(memory) = self.readonly.memory_items() {
            return Ok(memory.insert(item));
        }
        let stored = item.clone();
        self.execute_insert(&stored, item).await?;
        Ok(item.clone())
    }

    async fn execute_insert(&self, stored: &Row, target: &mut Row) -> Result<Response> {
        let record = self.readonly.reverse_convert(stored)?;
        let fields = self.params(&record, false)?;
        let statement = self.readonly.statements.insert(&fields);
        let response = self
            .readonly
            .command()
            .execute_statement(&statement, Some(&fields))
            .await?;
        if self.ids.len() == 1 {
            self.copy_inserted_id(target, &fields).await?;
        }
        Ok(response)
    }

    async fn copy_inserted_id(&self, item: &mut Row, fields: &[Parameter]) -> Result<()> {
        let column = self.ids[0].clone();
        let sql_column = self.readonly.column(&column);
        let reader = if let Some(map) = &self.return_sql {
            let built = ReturnStatement::new(&sql_column, fields);
            let mapped = ReturnStatement::text(Mapper::map(map.as_ref(), built.get_text()));
            self.readonly
                .command()
                .execute_reader_statement(&mapped, Some(fields))
                .await?
        } else {
            let inserted = self.readonly.statements.select_inserted_id(&sql_column);
            self.readonly
                .command()
                .execute_reader_statement(&inserted, None)
                .await?
        };
        if !reader.next().await? {
            return Ok(());
        }
        let current = reader
            .current()
            .ok_or_else(|| DatasourceError::message("Reader current must be a row"))?;
        let row = identified(named(current, &column, &sql_column), &column);
        let mapped = Mapper::map(
            &self.readonly.rows_mapper,
            self.readonly.row_converter.convert(&row)?,
        );
        self.write_id(item, &self.id_value(&mapped))
    }

    fn id_value(&self, item: &Row) -> Value {
        if self.ids.len() != 1 {
            return Value::Null;
        }
        item.get(&self.ids[0]).cloned().unwrap_or(Value::Null)
    }

    fn write_id(&self, item: &mut Row, id: &Value) -> Result<()> {
        if self.ids.len() != 1 || *id == Value::Null {
            return Ok(());
        }
        let name = self.ids[0].clone();
        let mut patch = Row::new();
        patch.insert(name.clone(), id.clone());
        let converted = self.readonly.row_converter.convert(&patch)?;
        if let Some(value) = converted.get(&name) {
            if *value != Value::Null {
                item.insert(name, value.clone());
            }
        }
        Ok(())
    }

    fn without_enrichment(&self, item: &Row) -> Row {
        let entries = self.readonly.enrichments.lock().expect("enrichments");
        item.iter()
            .filter(|(key, _)| !entries.iter().any(|entry| entry.enrichment.into == **key))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    pub(crate) fn enrichment_into(&self, name: &str) -> bool {
        self.readonly
            .enrichments
            .lock()
            .expect("enrichments")
            .iter()
            .any(|entry| entry.enrichment.into == name)
    }

    async fn save(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        if self.readonly.memory_items().is_some() {
            let stored = self.without_enrichment(item);
            return Ok(self.readonly.memory_items().expect("memory").save(&stored));
        }
        let record = self.readonly.reverse_convert(item)?;
        let fields = self.params(&record, false)?;
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let (owned, raw) = self.stored_predicate(&record, by_where)?;
        let predicate: &dyn IWhere = match (owned.as_ref(), raw) {
            (Some(clause), _) => clause,
            (None, Some(clause)) => clause,
            _ => unreachable!(),
        };
        let statement = self.readonly.statements.update(&fields, predicate);
        let parameters = crate::UpdateStatement::parameter_list(&fields, Some(predicate));
        self.readonly
            .command()
            .execute_statement(&statement, Some(&parameters))
            .await
    }

    async fn remove(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        if let Some(memory) = self.readonly.memory_items() {
            return Ok(memory.remove(item, by_where));
        }
        let record = self.readonly.reverse_convert(item)?;
        let (owned, raw) = self.stored_predicate(&record, by_where)?;
        let predicate: &dyn IWhere = match (owned.as_ref(), raw) {
            (Some(clause), _) => clause,
            (None, Some(clause)) => clause,
            _ => unreachable!(),
        };
        let statement = self.readonly.statements.delete(predicate);
        let parameters = predicate.parameters();
        let parameters = if parameters.is_empty() && statement.get_text().contains('$') {
            self.params(&record, true)?
        } else {
            parameters
        };
        self.readonly
            .command()
            .execute_statement(&statement, Some(&parameters))
            .await
    }

    fn stored_predicate<'a>(
        &'a self,
        record: &Row,
        by_where: Option<&'a dyn IWhere>,
    ) -> Result<(Option<Where>, Option<&'a dyn IWhere>)> {
        match by_where {
            Some(clause) => match clause.as_any().downcast_ref::<Where>() {
                Some(typed) => Ok((Some(self.store_clause(typed, false)?), None)),
                None => Ok((None, Some(clause))),
            },
            None => Ok((
                Some(self.store_clause(&self.id_where(record)?, true)?),
                None,
            )),
        }
    }

    fn store_clause(&self, clause: &Where, fields_applied: bool) -> Result<Where> {
        clause.map(
            |name| self.readonly.column(name),
            |name, cell| self.readonly.cell(name, cell, fields_applied),
        )
    }

    fn id_record(&self, id: &dyn IdKey) -> Result<Row> {
        if self.ids.is_empty() {
            return Err(DatasourceError::message(
                "ReadSingleById requires an Id column",
            ));
        }
        match (id.as_value(), id.as_row()) {
            (Some(value), _) if self.ids.len() == 1 => {
                let mut row = Row::new();
                row.insert(self.ids[0].clone(), value.clone());
                Ok(row)
            }
            (Some(_), _) => Err(DatasourceError::message(
                "Composite id requires a row keyed by each Id column",
            )),
            (_, Some(row)) => Ok(row.clone()),
            _ => Err(DatasourceError::message(
                "ReadSingleById requires an Id column",
            )),
        }
    }
}

impl Deref for CRUDRepository {
    type Target = ReadonlyRepository;

    fn deref(&self) -> &Self::Target {
        &self.readonly
    }
}

impl DerefMut for CRUDRepository {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.readonly
    }
}

impl IRepository for CRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.readonly.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for CRUDRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Option<Row>> {
        self.readonly.read_single(query, parameters).await
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Vec<Row>> {
        self.readonly.read(query, parameters).await
    }
}

#[async_trait]
impl ICRUDRepository for CRUDRepository {
    async fn create(&self, item: &mut Row) -> Result<Row> {
        Self::in_transaction(
            self.use_transaction(),
            Arc::clone(&self.readonly.connection),
            || self.create_core(item),
        )
        .await
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        Self::in_transaction(
            self.use_transaction(),
            Arc::clone(&self.readonly.connection),
            || self.update_core(item, by_where),
        )
        .await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        Self::in_transaction(
            self.use_transaction(),
            Arc::clone(&self.readonly.connection),
            || self.delete_core(item, by_where),
        )
        .await
    }

    fn id_names(&self) -> Vec<String> {
        self.ids.clone()
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        if let Some(memory) = self.readonly.memory_items() {
            let mut row = memory.by_id(id)?;
            if self.enriched() {
                if let Some(item) = row.take() {
                    let mut rows = vec![item];
                    self.readonly.fill(&mut rows, None).await?;
                    return Ok(rows.into_iter().next());
                }
            }
            return Ok(row);
        }
        let record = self.id_record(id)?;
        let clause = self.id_where(&record)?;
        self.read_single(Some(&clause), None).await
    }
}

fn named(mut row: Row, property: &str, sql_column: &str) -> Row {
    if property == sql_column {
        return row;
    }
    if let Some(cell) = row.remove(sql_column) {
        row.insert(property.to_string(), cell);
    }
    row
}

fn identified(row: Row, column: &str) -> Row {
    if row.contains_key(column) || row.len() != 1 {
        return row;
    }
    let value = row.into_values().next().unwrap_or(Value::Null);
    let mut named = Row::new();
    named.insert(column.to_string(), value);
    named
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
