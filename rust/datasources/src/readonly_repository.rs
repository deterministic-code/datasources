use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{MetaDataRowTypeConverter, RowTypeConverter, TypeConverter};
use crate::datasource_error::DatasourceError;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enrichment_via::EnrichmentVia;
use crate::entity::Entity;
use crate::i_repository::IRepository;
use crate::mappers::{IdentityMapper, Mapper};
use crate::mocks::MockConnection;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::readonly_repository_builder::ReadonlyRepositoryBuilder;
use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::repository::{map_stream, IReadonlyRepository};
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::shared_items::SharedItems;
use crate::statement::{Statement, StatementKind};
use crate::statement_mappers::StatementMappers;
use crate::stored_enrichment::StoredEnrichment;
use crate::value::Value;
use crate::IWhere;
use crate::SelectStatement;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::stream::try_unfold;
use futures_util::TryStreamExt;
use std::any::TypeId;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

enum ReadRun {
    Start,
    Buffered(std::vec::IntoIter<Row>),
    Source {
        reader: Box<dyn Reader>,
        mappers: StatementMappers,
    },
}

pub struct ReadonlyRepository<T: Entity = Row> {
    pub(crate) connection: Arc<dyn Connection>,
    pub(crate) middlewares: Vec<Arc<dyn CommandMiddleware>>,
    middleware_slot: Arc<Mutex<Vec<Arc<dyn CommandMiddleware>>>>,
    pub(crate) row_converter: Option<RowTypeConverter>,
    pub(crate) object_converter: Box<dyn TypeConverter<Row, Row> + Send + Sync>,
    entity_mapper: Arc<dyn Mapper<String, String> + Send + Sync>,
    property_mappings: Arc<dyn Mapper<String, String> + Send + Sync>,
    statement_mappers: StatementMappers,
    readonly_statements: ReadonlyRepositoryStatements,
    pub(crate) entity_type: TypeId,
    pub(crate) memory: Option<Arc<SharedItems>>,
    pub(crate) enrichments: Arc<Mutex<Vec<StoredEnrichment<T>>>>,
    pub(crate) schema: Schema,
    entity: PhantomData<T>,
}

#[derive(Clone, Copy)]
pub(crate) enum ReadQuery<'a> {
    All,
    Statement(&'a dyn Statement),
    Where(&'a dyn IWhere),
}

impl<'a> ReadQuery<'a> {
    fn statement(query: Option<&'a dyn Statement>) -> Self {
        query.map(Self::Statement).unwrap_or(Self::All)
    }

    pub(crate) fn clause(clause: &'a dyn IWhere) -> Self {
        Self::Where(clause)
    }

    fn memory_where(self) -> Option<&'a dyn IWhere> {
        match self {
            Self::Where(clause) => Some(clause),
            Self::Statement(statement) => statement.predicate(),
            Self::All => None,
        }
    }
}

impl<T: Entity> ReadonlyRepository<T> {
    pub fn new(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryMappingOptions,
    ) -> Result<Self> {
        Ok(Self::from_built(ReadonlyRepositoryBuilder::parts(
            connection, options,
        )?))
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn from_built(built: BuiltRepository) -> Self {
        let enrichments = Arc::new(Mutex::new(Vec::new()));
        Self {
            connection: built.connection,
            middlewares: Vec::new(),
            middleware_slot: Arc::new(Mutex::new(Vec::new())),
            row_converter: built.row_converter,
            object_converter: built.object_converter,
            entity_mapper: Arc::clone(&built.entity_mapper),
            property_mappings: Arc::clone(&built.property_mappings),
            statement_mappers: StatementMappers::new(built.entity_mapper, built.property_mappings),
            readonly_statements: built.statements,
            entity_type: TypeId::of::<T>(),
            memory: None,
            enrichments,
            schema: built.schema,
            entity: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn memory(items: Vec<Row>) -> Self {
        let enrichments = Arc::new(Mutex::new(Vec::new()));
        let entity: Arc<dyn Mapper<String, String> + Send + Sync> =
            Arc::new(IdentityMapper::<String>::new());
        let property: Arc<dyn Mapper<String, String> + Send + Sync> =
            Arc::new(IdentityMapper::<String>::new());
        Self {
            connection: MockConnection::new(),
            middlewares: Vec::new(),
            middleware_slot: Arc::new(Mutex::new(Vec::new())),
            row_converter: None,
            object_converter: Box::new(MetaDataRowTypeConverter::new()),
            entity_mapper: Arc::clone(&entity),
            property_mappings: Arc::clone(&property),
            statement_mappers: StatementMappers::new(Arc::clone(&entity), Arc::clone(&property)),
            readonly_statements: ReadonlyRepositoryStatements::new(
                "",
                Vec::new(),
                StatementMappers::new(entity, property),
            ),
            entity_type: TypeId::of::<T>(),
            memory: Some(SharedItems::new(items)),
            enrichments,
            schema: Schema::default(),
            entity: PhantomData,
        }
    }

    pub fn add_enriched<TRelated: Entity>(
        &mut self,
        enrichment: crate::enrichment::Enrichment,
        via: impl EnrichmentVia<TRelated>,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy<T, TRelated>>>,
    ) -> &mut Self {
        let strategy = strategy.unwrap_or_else(|| Arc::new(DefaultReadonlyEnrichmentStrategy));
        let via = via.read_via();
        self.enrichments
            .lock()
            .expect("enrichments")
            .push(StoredEnrichment::direct(enrichment, via, None, strategy));
        self
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        push_middleware(self, middleware);
    }

    pub(crate) fn enriched(&self) -> bool {
        self.enriched_with(None)
    }

    fn enriched_with(&self, options: Option<&ReadonlyRepositoryMappingOptions>) -> bool {
        options.and_then(|opts| opts.enrichment.as_ref()).is_some()
            || !self.enrichments.lock().expect("enrichments").is_empty()
    }

    pub(crate) fn memory_items(&self) -> Option<&Arc<SharedItems>> {
        self.memory.as_ref()
    }

    pub(crate) fn command(&self) -> Arc<dyn Command> {
        let command = self.connection.create_command();
        let extra = self.middleware_slot.lock().expect("middleware").clone();
        let middlewares: Vec<_> = self.middlewares.iter().cloned().chain(extra).collect();
        for middleware in &middlewares {
            command.add_middleware(middleware.clone());
        }
        command.set_schema(Some(Schema::new(
            self.schema
                .columns
                .iter()
                .flat_map(|(name, column)| {
                    [
                        (name.clone(), column.clone()),
                        (
                            Mapper::map(self.property_mappings.as_ref(), name.clone()),
                            column.clone(),
                        ),
                    ]
                })
                .collect(),
        )));
        command
    }

    pub(crate) fn reverse_convert(&self, item: &Row) -> Result<Row> {
        let row = self.object_converter.reverse_convert(item)?;
        match &self.row_converter {
            Some(converter) => converter.reverse_convert(&row),
            None => Ok(row),
        }
    }

    pub(crate) fn column(&self, property: &str) -> String {
        self.statement_mappers.column(property)
    }

    pub(crate) async fn fill(
        &self,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let entries = self.enrichments.lock().expect("enrichments").clone();
        for entry in entries {
            entry.apply(parents, sources).await?;
        }
        Ok(())
    }

    pub(crate) fn cell(
        &self,
        property: &str,
        value: &Value,
        fields_applied: bool,
    ) -> Result<Value> {
        self.cell_with(property, value, fields_applied, None)
    }

    fn cell_with(
        &self,
        property: &str,
        value: &Value,
        fields_applied: bool,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Value> {
        let rows = options
            .and_then(|opts| opts.row_converter.as_ref())
            .or(self.row_converter.as_ref());
        match rows {
            Some(converter) => match converter.fields.get(property) {
                Some(field) if !fields_applied => field.reverse_convert(value),
                _ => Ok(value.clone()),
            },
            None => Ok(value.clone()),
        }
    }

    fn mappers(&self, options: Option<&ReadonlyRepositoryMappingOptions>) -> StatementMappers {
        match options {
            Some(opts) if opts.entity_mapper.is_some() || opts.property_mappings.is_some() => {
                StatementMappers::new(
                    opts.entity_mapper
                        .clone()
                        .unwrap_or_else(|| Arc::clone(&self.entity_mapper)),
                    opts.property_mappings
                        .clone()
                        .unwrap_or_else(|| Arc::clone(&self.property_mappings)),
                )
            }
            _ => self.statement_mappers.clone(),
        }
    }

    fn objects<'a>(
        &'a self,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> &'a dyn TypeConverter<Row, Row> {
        options
            .and_then(|opts| opts.object_converter.as_deref())
            .unwrap_or(self.object_converter.as_ref())
    }

    fn rows<'a>(
        &'a self,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Option<&'a RowTypeConverter> {
        options
            .and_then(|opts| opts.row_converter.as_ref())
            .or(self.row_converter.as_ref())
    }

    async fn read_tracked(
        &self,
        query: ReadQuery<'_>,
        parameters: Option<&[Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<(Row, Option<Row>)>> {
        self.apply(self.read_items(query, parameters, options).await?)
            .await
    }

    async fn read_items(
        &self,
        query: ReadQuery<'_>,
        parameters: Option<&[Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<(Row, Option<Row>)>> {
        let mappers = self.mappers(options);
        let sources = self
            .fetch_renamed(query, parameters, &mappers, options)
            .await?;
        if self.memory.is_some() {
            return Ok(sources
                .into_iter()
                .map(|row| {
                    let source = row.clone();
                    (row, Some(source))
                })
                .collect());
        }
        let objects = self.objects(options);
        let rows = self.rows(options);
        let mut pairs = Vec::new();
        for renamed in sources {
            let cells = match rows {
                Some(converter) => converter.convert(&renamed)?,
                None => renamed.clone(),
            };
            let item = objects.convert(&cells)?;
            pairs.push((item, Some(renamed)));
        }
        Ok(pairs)
    }

    async fn fetch_renamed(
        &self,
        query: ReadQuery<'_>,
        parameters: Option<&[Parameter]>,
        mappers: &StatementMappers,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<Row>> {
        if let Some(memory) = &self.memory {
            return Ok(memory.filter(query.memory_where()));
        }
        let reader = self.reader_for(query, parameters, mappers, options).await?;
        self.rows_from(reader, mappers).await
    }

    async fn ensure_open(&self) -> Result<()> {
        if self.connection.is_open() {
            return Ok(());
        }
        self.connection.open().await
    }

    async fn reader_for(
        &self,
        query: ReadQuery<'_>,
        parameters: Option<&[Parameter]>,
        mappers: &StatementMappers,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Box<dyn Reader>> {
        self.ensure_open().await?;
        if let ReadQuery::Where(where_clause) = query {
            let (statement, params) = self.stored_query(where_clause, mappers, options)?;
            return self
                .command()
                .execute_reader_statement(&statement, Some(parameters.unwrap_or(&params)))
                .await;
        }
        let statement = match query {
            ReadQuery::Statement(statement) => Some(statement),
            _ => None,
        };
        match statement {
            Some(statement) if statement.kind() == StatementKind::Procedure => {
                let stored = statement
                    .as_any()
                    .downcast_ref::<crate::statements::ExecuteStoredProcedureStatement>();
                let from_statement =
                    stored.map(crate::statements::ExecuteStoredProcedureStatement::parameters);
                self.command()
                    .execute_reader_statement(
                        statement,
                        Some(parameters.or(from_statement).unwrap_or(&[])),
                    )
                    .await
            }
            Some(statement) => match statement.as_any().downcast_ref::<SelectStatement>() {
                Some(select) if !select.table.is_empty() => {
                    let mapped = mappers.apply_select(select, &|name, cell| {
                        self.cell_with(name, cell, false, options)
                    })?;
                    let params = parameters.map(|items| items.to_vec()).unwrap_or_else(|| {
                        mapped
                            .predicate()
                            .map(|predicate| predicate.parameters())
                            .unwrap_or_default()
                    });
                    self.command()
                        .execute_reader_statement(&mapped, Some(&params))
                        .await
                }
                _ => {
                    let params = parameters.map(|items| items.to_vec()).unwrap_or_else(|| {
                        statement
                            .predicate()
                            .map(|predicate| predicate.parameters())
                            .unwrap_or_default()
                    });
                    self.command()
                        .execute_reader_statement(statement, Some(&params))
                        .await
                }
            },
            None => {
                let statement =
                    self.own(self.readonly_statements.select(None), mappers, options)?;
                self.command()
                    .execute_reader_statement(&statement, Some(parameters.unwrap_or(&[])))
                    .await
            }
        }
    }

    async fn rows_from(
        &self,
        reader: Box<dyn Reader>,
        mappers: &StatementMappers,
    ) -> Result<Vec<Row>> {
        let mut rows = Vec::new();
        while reader.next().await? {
            let row = reader
                .current()
                .ok_or_else(|| DatasourceError::message("Reader current must be a row"))?;
            rows.push(self.column_to_property_name(row, mappers));
        }
        Ok(rows)
    }

    fn convert_renamed(
        &self,
        renamed: Row,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Row> {
        if self.memory.is_some() {
            return Ok(renamed);
        }
        let cells = match self.rows(options) {
            Some(converter) => converter.convert(&renamed)?,
            None => renamed,
        };
        self.objects(options).convert(&cells)
    }

    async fn next_source(
        &self,
        reader: Box<dyn Reader>,
        mappers: StatementMappers,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<(Row, ReadRun)>> {
        if !reader.next().await? {
            return Ok(None);
        }
        let current = reader
            .current()
            .ok_or_else(|| DatasourceError::message("Reader current must be a row"))?;
        let renamed = self.column_to_property_name(current, &mappers);
        let item = self.convert_renamed(renamed, options)?;
        Ok(Some((item, ReadRun::Source { reader, mappers })))
    }

    async fn apply(&self, pairs: Vec<(Row, Option<Row>)>) -> Result<Vec<(Row, Option<Row>)>> {
        Ok(pairs)
    }

    async fn fill_rows(
        &self,
        pairs: Vec<(Row, Option<Row>)>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<T>> {
        let (mut rows, sources): (Vec<Row>, Vec<Option<Row>>) = pairs.into_iter().unzip();
        if let Some(strategy) = options.and_then(|opts| opts.enrichment.clone()) {
            let via = Arc::new(ReadonlyRepository::<Row>::memory(Vec::new()));
            strategy
                .apply(
                    via,
                    &crate::Enrichment::new(
                        "",
                        "",
                        "",
                        None,
                        crate::EnrichmentPropertyType::Object,
                    ),
                    &mut rows,
                    Some(&sources),
                )
                .await?;
            return rows.into_iter().map(nested_item).collect();
        }
        let mut items = rows
            .into_iter()
            .map(crate::repository::map_row)
            .collect::<Result<Vec<_>>>()?;
        self.fill(&mut items, Some(&sources)).await?;
        Ok(items)
    }

    fn column_to_property_name(&self, row: Row, mappers: &StatementMappers) -> Row {
        row.into_iter()
            .map(|(key, value)| (mappers.property(&key), value))
            .collect()
    }

    fn own(
        &self,
        select: SelectStatement,
        mappers: &StatementMappers,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<SelectStatement> {
        match select.table.is_empty() {
            true => Ok(select),
            false => mappers.apply_select(&select, &|name, cell| {
                self.cell_with(name, cell, false, options)
            }),
        }
    }

    fn stored_query(
        &self,
        where_clause: &dyn IWhere,
        mappers: &StatementMappers,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<(SelectStatement, Vec<Parameter>)> {
        let mapped = self.own(
            self.readonly_statements.select(Some(where_clause)),
            mappers,
            options,
        )?;
        let params = mapped
            .predicate()
            .map(|clause| clause.parameters())
            .unwrap_or_default();
        Ok((mapped, params))
    }

    pub async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        IReadonlyRepository::read_single(self, query, options).await
    }

    pub fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        IReadonlyRepository::read(self, query, options)
    }

    pub async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        IReadonlyRepository::read_single_where(self, clause, options).await
    }

    pub fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        IReadonlyRepository::read_where(self, clause, options)
    }
}

impl<T: Entity> IRepository for ReadonlyRepository<T> {
    fn entity_type(&self) -> TypeId {
        self.entity_type
    }

    fn schema(&self) -> &Schema {
        &self.schema
    }
}

impl<T: Entity> ReadonlyRepository<T> {
    pub(crate) fn read_rows<'a>(
        &'a self,
        query: ReadQuery<'a>,
        parameters: Option<&'a [Parameter]>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<Row>> + Send + 'a>> {
        Box::pin(try_unfold(ReadRun::Start, move |run| async move {
            match run {
                ReadRun::Start => {
                    if self.enriched_with(options) {
                        let items: Vec<Row> = self
                            .read_tracked(query, parameters, options)
                            .await?
                            .into_iter()
                            .map(|(item, _)| item)
                            .collect();
                        return next_buffered(items);
                    }
                    if let Some(memory) = &self.memory {
                        let items = memory
                            .filter(query.memory_where())
                            .into_iter()
                            .map(|row| self.convert_renamed(row, options))
                            .collect::<Result<Vec<_>>>()?;
                        return next_buffered(items);
                    }
                    let mappers = self.mappers(options);
                    let reader = self
                        .reader_for(query, parameters, &mappers, options)
                        .await?;
                    self.next_source(reader, mappers, options).await
                }
                ReadRun::Buffered(items) => next_buffered_iter(items),
                ReadRun::Source { reader, mappers } => {
                    self.next_source(reader, mappers, options).await
                }
            }
        }))
    }
}

#[async_trait]
impl<T: Entity> IReadonlyRepository<T> for ReadonlyRepository<T> {
    async fn read_single(
        &self,
        query: Option<&dyn Statement>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        self.read(query, options).try_next().await
    }

    fn read<'a>(
        &'a self,
        query: Option<&'a dyn Statement>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        self.read_query(ReadQuery::statement(query), options)
    }

    async fn read_single_where(
        &self,
        clause: &dyn IWhere,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<T>> {
        self.read_where(clause, options).try_next().await
    }

    fn read_where<'a>(
        &'a self,
        clause: &'a dyn IWhere,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        self.read_query(ReadQuery::clause(clause), options)
    }
}

impl<T: Entity> ReadonlyRepository<T> {
    fn read_query<'a>(
        &'a self,
        query: ReadQuery<'a>,
        options: Option<&'a ReadonlyRepositoryMappingOptions>,
    ) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + 'a>> {
        if !self.enriched_with(options) {
            return map_stream(self.read_rows(query, None, options));
        }
        Box::pin(try_unfold(None, move |state| async move {
            match state {
                None => {
                    let pairs = self.read_tracked(query, None, options).await?;
                    next_entities(self.fill_rows(pairs, options).await?)
                }
                Some(items) => next_entities_iter(items),
            }
        }))
    }
}

fn next_entities<T>(items: Vec<T>) -> Result<Option<(T, Option<std::vec::IntoIter<T>>)>> {
    next_entities_iter(items.into_iter())
}

fn next_entities_iter<T>(
    mut items: std::vec::IntoIter<T>,
) -> Result<Option<(T, Option<std::vec::IntoIter<T>>)>> {
    Ok(items.next().map(|item| (item, Some(items))))
}

fn next_buffered(items: Vec<Row>) -> Result<Option<(Row, ReadRun)>> {
    next_buffered_iter(items.into_iter())
}

fn next_buffered_iter(mut items: std::vec::IntoIter<Row>) -> Result<Option<(Row, ReadRun)>> {
    Ok(items.next().map(|row| (row, ReadRun::Buffered(items))))
}

fn nested_item<T: Entity>(row: Row) -> Result<T> {
    let mut item: T = crate::repository::map_row(row.clone())?;
    item.hydrate_from_row(&row)?;
    Ok(item)
}

pub(crate) fn push_middleware<T: Entity>(
    repo: &ReadonlyRepository<T>,
    middleware: Arc<dyn CommandMiddleware>,
) {
    repo.middleware_slot
        .lock()
        .expect("middleware")
        .push(middleware);
}
