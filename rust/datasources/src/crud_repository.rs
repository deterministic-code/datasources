use crate::built_repository::BuiltRepository;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_crud_repository_options::create_crud_repository_options;
use crate::crud_repository_builder::{write_statements, CRUDRepositoryBuilder};
use crate::crud_repository_options::CRUDRepositoryOptions;
use crate::crud_repository_statements::CRUDRepositoryStatements;
use crate::datasource_error::DatasourceError;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
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
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::stubs::StubRepository;
use crate::unit_of_work_transaction::UnitOfWorkTransaction;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::write_mode::WriteMode;
use crate::{Expression, Literal, Operator, Where};
use async_trait::async_trait;
use futures_core::Stream;
use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::Arc;

pub struct CRUDRepository {
    readonly: ReadonlyRepository,
    ids: Vec<String>,
    create_properties: Option<Vec<String>>,
    update_properties: Option<Vec<String>>,
    crud_statements: CRUDRepositoryStatements,
    tree: RepositoryTree,
    write: Arc<dyn WriteEnrichmentStrategy>,
}

fn tree_for(readonly: &ReadonlyRepository) -> RepositoryTree {
    RepositoryTree::new(RepositoryTreeNode::new(
        Arc::new(StubRepository::new("crud")),
        "",
        readonly.enrichments.clone(),
    ))
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
        let write = options.write.clone();
        let create_properties = options.create_properties.clone();
        let update_properties = options.update_properties.clone();
        let built = CRUDRepositoryBuilder::parts(connection, options)?;
        let crud_statements = write_statements(&built.statements);
        let mut repo = Self::from_built(built, crud_statements, create_properties, update_properties)?;
        if let Some(write) = write {
            repo.write = write;
        }
        Ok(repo)
    }

    fn from_built(
        built: BuiltRepository,
        crud_statements: CRUDRepositoryStatements,
        create_properties: Option<Vec<String>>,
        update_properties: Option<Vec<String>>,
    ) -> Result<Self> {
        let id = built.ids.clone();
        let readonly = ReadonlyRepository::from_built(built);
        let tree = tree_for(&readonly);
        Ok(Self {
            readonly,
            crud_statements,
            ids: id,
            create_properties,
            update_properties,
            tree,
            write: Arc::new(DefaultWriteEnrichmentStrategy::default()),
        })
    }

    #[allow(dead_code)]
    pub(crate) fn memory(items: Vec<Row>) -> Self {
        Self::memory_with(items, None)
    }

    #[allow(dead_code)]
    pub(crate) fn memory_with(items: Vec<Row>, write: Option<Arc<dyn WriteEnrichmentStrategy>>) -> Self {
        let ids = vec!["id".into()];
        let readonly = ReadonlyRepository::memory(items);
        let tree = tree_for(&readonly);
        Self {
            readonly,
            crud_statements: CRUDRepositoryStatements::empty(),
            ids,
            create_properties: None,
            update_properties: None,
            tree,
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

    pub fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        crate::push_middleware(&self.readonly, middleware);
    }

    fn enriched(&self) -> bool {
        self.readonly.enriched()
    }

    fn use_transaction(&self) -> bool {
        self.write.use_transaction()
    }

    async fn ensure_open(&self) -> Result<()> {
        if self.readonly.memory_items().is_some() || self.readonly.connection.is_open() {
            return Ok(());
        }
        self.readonly.connection.open().await
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
        self.write
            .resolve_lookups(&self.tree, std::slice::from_mut(item))
            .await?;
        if !self.enriched() {
            return self.inserted(item).await;
        }
        if let Some(memory) = self.readonly.memory_items().cloned() {
            let mut stored = self.without_enrichment(item);
            let created = memory.insert(&mut stored);
            if let [column] = self.ids.as_slice() {
                if let Some(id) = created.get(column).filter(|id| **id != Value::Null) {
                    item.insert(column.clone(), id.clone());
                }
            }
        } else {
            let stored = item.clone();
            self.execute_insert(&stored, item).await?;
        }
        self.write
            .persist(&self.tree, std::slice::from_mut(item), WriteMode::Create)
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
        let mut resolved = item.clone();
        self.write
            .resolve_lookups(&self.tree, std::slice::from_mut(&mut resolved))
            .await?;
        let response = self.save(&resolved, by_where).await?;
        if self.enriched() {
            let mut parent = item.clone();
            self.write
                .persist(
                    &self.tree,
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
                    &self.tree,
                    std::slice::from_mut(&mut parent),
                    WriteMode::Delete,
                )
                .await?;
        }
        self.remove(item, by_where).await
    }

    fn params(&self, record: &Row, include_ids: bool) -> Result<Vec<Parameter>> {
        self.fields(record, |key, _| {
            let fallback = include_ids || !self.ids.iter().any(|id| id == key);
            if include_ids {
                fallback
            } else {
                listed(&self.update_properties, key, fallback)
            }
        })
    }

    fn insert_params(&self, record: &Row) -> Result<Vec<Parameter>> {
        self.fields(record, |key, value| {
            listed(
                &self.create_properties,
                key,
                !self.ids.iter().any(|id| id == key) || assigned(value),
            )
        })
    }

    fn fields(
        &self,
        record: &Row,
        include: impl Fn(&str, &Value) -> bool,
    ) -> Result<Vec<Parameter>> {
        let entries = self.readonly.enrichments.lock().expect("enrichments");
        let columns = self.crud_statements.read_columns();
        let extras = |key: &str| {
            entries.iter().any(|entry| {
                entry.enrichment.r#type == crate::EnrichmentPropertyType::Object
                    && entry.enrichment.association.is_none()
                    && entry.enrichment.source_property == key
            })
        };
        let mut pairs: Vec<_> = record
            .iter()
            .filter(|(key, value)| {
                !matches!(value, Value::Null)
                    && (extras(key)
                        || include(key, value)
                            && (columns.is_empty()
                                || columns.iter().any(|name| {
                                    name == *key || self.readonly.column(name) == **key
                                }))
                            && !entries
                                .iter()
                                .any(|entry| entry.enrichment.destination_property == **key))
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        drop(entries);
        pairs.sort_by(|left, right| left.0.cmp(&right.0));
        pairs
            .into_iter()
            .map(|(name, value)| {
                Ok(Parameter::new(
                    name.clone(),
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
        let fields = self.insert_params(&record)?;
        let statement = self.crud_statements.insert(&fields);
        let response = self
            .readonly
            .command()
            .execute_statement(&statement, Some(statement.fields()))
            .await?;
        if self.ids.len() == 1 && !fields.iter().any(|field| field.name == self.ids[0]) {
            self.copy_inserted_id(target, statement.fields()).await?;
        }
        Ok(response)
    }

    async fn copy_inserted_id(&self, item: &mut Row, fields: &[Parameter]) -> Result<()> {
        let _ = fields;
        let column = self.ids[0].clone();
        let inserted = self.crud_statements.select_inserted_id(&column);
        let sql_column = inserted.column.clone();
        let reader = self
            .readonly
            .command()
            .execute_reader_statement(&inserted, None)
            .await?;
        if !reader.next().await? {
            return Ok(());
        }
        let current = reader
            .current()
            .ok_or_else(|| DatasourceError::message("Reader current must be a row"))?;
        let row = identified(named(current, &column, &sql_column), &column);
        let cells = match &self.readonly.row_converter {
            Some(converter) => converter.convert(&row)?,
            None => row,
        };
        let mapped = self.readonly.object_converter.convert(&cells)?;
        if let Some(value) = mapped.get(&column).filter(|value| **value != Value::Null) {
            item.insert(column, value.clone());
        }
        Ok(())
    }

    fn id_value(&self, item: &Row) -> Value {
        if self.ids.len() != 1 {
            return Value::Null;
        }
        item.get(&self.ids[0]).cloned().unwrap_or(Value::Null)
    }

    fn without_enrichment(&self, item: &Row) -> Row {
        let entries = self.readonly.enrichments.lock().expect("enrichments");
        item.iter()
            .filter(|(key, _)| {
                !entries
                    .iter()
                    .any(|entry| entry.enrichment.destination_property == **key)
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }

    #[allow(dead_code)]
    pub(crate) fn enrichment_into(&self, name: &str) -> bool {
        self.readonly
            .enrichments
            .lock()
            .expect("enrichments")
            .iter()
            .any(|entry| entry.enrichment.destination_property == name)
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
        let owned;
        let predicate: &dyn IWhere = match by_where {
            Some(clause) => clause,
            None => {
                owned = self.id_where(&record)?;
                &owned
            }
        };
        let applied = by_where.is_none();
        let statements = &self.crud_statements;
        let statement = statements.update(&fields, predicate, &|name, value| {
            self.readonly.cell(name, value, applied)
        })?;
        let parameters = statement.parameters();
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
        let owned;
        let predicate: &dyn IWhere = match by_where {
            Some(clause) => clause,
            None => {
                owned = self.id_where(&record)?;
                &owned
            }
        };
        let applied = by_where.is_none();
        let statements = &self.crud_statements;
        let statement = statements.delete(predicate, &|name, value| {
            self.readonly.cell(name, value, applied)
        })?;
        let parameters = statement
            .where_clause()
            .map(|clause| clause.parameters())
            .unwrap_or_default();
        let parameters = if parameters.is_empty()
            && crate::DefaultStatementTextRenderer::instance()
                .render(&statement)
                .contains('$')
        {
            self.params(&record, true)?
        } else {
            parameters
        };
        self.readonly
            .command()
            .execute_statement(&statement, Some(&parameters))
            .await
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
        options: Option<&crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        self.readonly.read_single(query, parameters, options).await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn ReadInput>,
        parameters: Option<&'a [Parameter]>,
        options: Option<&'a crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        self.readonly.read(query, parameters, options)
    }
}

#[async_trait]
impl ICRUDRepository for CRUDRepository {
    async fn create(&self, item: &mut Row) -> Result<Row> {
        self.ensure_open().await?;
        Self::in_transaction(
            self.use_transaction(),
            Arc::clone(&self.readonly.connection),
            || self.create_core(item),
        )
        .await
    }

    async fn update(&self, item: &mut Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.ensure_open().await?;
        Self::in_transaction(
            self.use_transaction(),
            Arc::clone(&self.readonly.connection),
            || self.update_core(item, by_where),
        )
        .await
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn IWhere>) -> Result<Response> {
        self.ensure_open().await?;
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

    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        crate::push_middleware(&self.readonly, middleware);
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
        self.read_single(Some(&clause), None, None).await
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

fn listed(list: &Option<Vec<String>>, key: &str, fallback: bool) -> bool {
    match list {
        Some(names) => names.iter().any(|name| name == key),
        None => fallback,
    }
}

fn assigned(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Text(text) => !text.is_empty(),
        Value::Int(0) => false,
        _ => true,
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
