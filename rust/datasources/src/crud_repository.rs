use crate::built_repository::BuiltRepository;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_options::create_repository_options;
use crate::crud_repository_options::{identity_mapper, CRUDRepositoryOptions};
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::{
    copy_ids, strip_enriched, DefaultWriteEnrichmentStrategy,
};
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
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
use crate::shared_items::SharedItems;
use crate::table_repository_builder::RepositoryBuilder;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::write_mode::WriteMode;
use crate::{Expression, Literal, Operator, Text, Where};
use async_trait::async_trait;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub struct CRUDRepository {
    pub(crate) readonly: ReadonlyRepository,
    create: TransformMapper<Value, Row>,
    ids: Vec<String>,
    write: Arc<dyn WriteEnrichmentStrategy>,
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
        let mut options = create_repository_options::<T>();
        options.row_converter = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(connection, options)?;
        repo.readonly.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        Ok(Self::from_built(RepositoryBuilder::parts_crud(
            connection, options,
        )?))
    }

    pub fn from_built(mut built: BuiltRepository) -> Self {
        let id = built.ids.clone();
        let create = built.create.take().unwrap_or_else(|| identity_mapper(&id));
        let write = built
            .write
            .take()
            .unwrap_or_else(|| Arc::new(DefaultWriteEnrichmentStrategy::default()));
        Self {
            readonly: ReadonlyRepository::from_built(built),
            create,
            ids: id,
            write,
        }
    }

    pub fn memory(items: Vec<Row>) -> Self {
        Self::memory_with(items, None)
    }

    pub fn memory_with(items: Vec<Row>, write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> Self {
        let ids = vec!["id".into()];
        Self {
            readonly: ReadonlyRepository::memory(items),
            create: identity_mapper(&ids),
            ids,
            write: write.unwrap_or_else(|| Arc::new(DefaultWriteEnrichmentStrategy::default())),
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
        self.readonly.add_enriched(enrichment, via, strategy);
        self
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.readonly.add_middleware(middleware);
    }

    pub fn items(&self) -> Option<&Arc<SharedItems>> {
        self.readonly.memory_items()
    }

    fn enriched(&self) -> bool {
        self.readonly.enriched()
    }

    fn params(&self, record: &Row, include_ids: bool) -> Result<Vec<Parameter>> {
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, _)| include_ids || !self.ids.contains(*key))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
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

    pub(crate) async fn inserted(&self, item: &mut Row) -> Result<Row> {
        if let Some(memory) = self.readonly.memory_items() {
            return Ok(memory.insert(item));
        }
        self.insert(item).await?;
        Ok(item.clone())
    }

    async fn insert(&self, item: &mut Row) -> Result<Response> {
        let record = self.readonly.reverse_convert(item)?;
        let fields = self.params(&record, false)?;
        let statement = self.readonly.statements.insert(&fields);
        let response = self
            .readonly
            .command()
            .execute_statement(&statement, Some(&fields))
            .await?;
        if response.result != Value::Null {
            apply(item, Mapper::map(&self.create, response.result.clone()));
        }
        Ok(response)
    }

    pub(crate) async fn save(
        &self,
        item: &Row,
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
        if let Some(memory) = self.readonly.memory_items() {
            return Ok(memory.save(item));
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

    pub(crate) async fn remove(
        &self,
        item: &Row,
        by_where: Option<&dyn IWhere>,
    ) -> Result<Response> {
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
            Some(clause) => match self.readonly.store_where(clause, false)? {
                Some(stored) => Ok((Some(stored), None)),
                None => Ok((None, Some(clause))),
            },
            None => Ok((
                Some(self.readonly.store_clause(&self.id_where(record)?, true)?),
                None,
            )),
        }
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
        if !self.enriched() {
            return self.inserted(item).await;
        }
        let mut stored = {
            let entries = self.readonly.enrichments.lock().expect("enrichments");
            strip_enriched(item, &entries)
        };
        let created = self.inserted(&mut stored).await?;
        copy_ids(item, &created, &self.ids);
        self.write
            .persist(
                &self.readonly.tree,
                std::slice::from_mut(item),
                WriteMode::Create,
            )
            .await?;
        if self.ids.len() != 1 {
            return Ok(item.clone());
        }
        let Some(id) = item.get(&self.ids[0]).cloned() else {
            return Ok(item.clone());
        };
        if matches!(id, Value::Null) {
            return Ok(item.clone());
        }
        Ok(self
            .read_single_by_id(&id)
            .await?
            .unwrap_or_else(|| item.clone()))
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        let stored = if self.enriched() {
            let entries = self.readonly.enrichments.lock().expect("enrichments");
            strip_enriched(item, &entries)
        } else {
            item.clone()
        };
        let response = self.save(&stored, by_where).await?;
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

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
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

fn apply(item: &mut Row, identity: Row) {
    for (key, value) in identity {
        if value != Value::Null {
            item.insert(key, value);
        }
    }
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
