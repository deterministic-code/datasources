use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, MetaDataRowTypeConverter, RowTypeConverter, TypeConverter};
use crate::create_readonly_repository_mapping_options::create_readonly_repository_mapping_options;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::mappers::{IdentityMapper, Mapper};
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::select_statement::SelectStatement;
use crate::shared_items::SharedItems;
use crate::statement::{Statement, StatementKind};
use crate::statements::Where;
use crate::statement_mappers::StatementMappers;
use crate::stored_enrichment::StoredEnrichment;
use crate::stubs::StubConnection;
use crate::readonly_repository_builder::ReadonlyRepositoryBuilder;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::value::Value;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::{Arc, Mutex};

pub struct ReadonlyRepository {
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
    pub(crate) enrichments: Arc<Mutex<Vec<StoredEnrichment>>>,
}

impl ReadonlyRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_readonly_repository_mapping_options::<T>();
        options.row_converter = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(connection, options)?;
        repo.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryMappingOptions,
    ) -> Result<Self> {
        Ok(Self::from_built(ReadonlyRepositoryBuilder::parts(
            connection, options,
        )?))
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
            entity_type: TypeId::of::<Row>(),
            memory: None,
            enrichments,
        }
    }

    pub fn memory(items: Vec<Row>) -> Self {
        let enrichments = Arc::new(Mutex::new(Vec::new()));
        let entity: Arc<dyn Mapper<String, String> + Send + Sync> =
            Arc::new(IdentityMapper::<String>::new());
        let property: Arc<dyn Mapper<String, String> + Send + Sync> =
            Arc::new(IdentityMapper::<String>::new());
        Self {
            connection: Arc::new(StubConnection::new("memory")),
            middlewares: Vec::new(),
            middleware_slot: Arc::new(Mutex::new(Vec::new())),
            row_converter: None,
            object_converter: Box::new(MetaDataRowTypeConverter::new(Vec::new())),
            entity_mapper: Arc::clone(&entity),
            property_mappings: Arc::clone(&property),
            statement_mappers: StatementMappers::new(Arc::clone(&entity), Arc::clone(&property)),
            readonly_statements: ReadonlyRepositoryStatements::new(
                "",
                Vec::new(),
                StatementMappers::new(entity, property),
            ),
            entity_type: TypeId::of::<Row>(),
            memory: Some(SharedItems::new(items)),
            enrichments,
        }
    }

    pub fn add_enriched(
        &mut self,
        enrichment: crate::enrichment::Enrichment,
        via: impl EnrichmentVia,
        strategy: Option<Arc<dyn ReadonlyEnrichmentStrategy>>,
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
        !self.enrichments.lock().expect("enrichments").is_empty()
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
        command
    }

    pub(crate) fn reverse_convert(&self, item: &Row) -> Result<Row> {
        let row = self.object_converter.reverse_convert(item)?;
        match &self.row_converter {
            Some(converter) => converter.reverse_convert(&row),
            None => Ok(row),
        }
    }

    fn store_where(
        &self,
        where_clause: &dyn IWhere,
        fields_applied: bool,
        mappers: &StatementMappers,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Where>> {
        match where_clause.as_any().downcast_ref::<Where>() {
            Some(clause) => Ok(Some(mappers.apply_where(clause, &|name, cell| {
                self.cell_with(name, cell, fields_applied, options)
            })?)),
            None => Ok(None),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn column(&self, property: &str) -> String {
        self.statement_mappers.column(property)
    }

    pub(crate) async fn fill(
        &self,
        parents: &mut Vec<Row>,
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
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<(Row, Option<Row>)>> {
        self.apply(self.read_items(query, parameters, options).await?)
            .await
    }

    async fn read_items(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<(Row, Option<Row>)>> {
        let mappers = self.mappers(options);
        let sources = self.fetch_renamed(query, parameters, &mappers, options).await?;
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
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
        mappers: &StatementMappers,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<Row>> {
        if let Some(memory) = &self.memory {
            return Ok(memory.filter_input(query));
        }
        if let Some(where_clause) = query.and_then(ReadInput::as_where) {
            let (statement, params) = self.stored_query(where_clause, mappers, options)?;
            return self
                .fetch_sources(&statement, parameters.unwrap_or(&params).to_vec(), mappers)
                .await;
        }
        let statement = query.and_then(ReadInput::as_statement);
        match statement {
            Some(statement) if statement.kind() == StatementKind::Procedure => {
                self.fetch_sources(statement, parameters.unwrap_or(&[]).to_vec(), mappers)
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
                    self.fetch_sources(&mapped, params, mappers).await
                }
                _ => {
                    let params = parameters.map(|items| items.to_vec()).unwrap_or_else(|| {
                        statement
                            .predicate()
                            .map(|predicate| predicate.parameters())
                            .unwrap_or_default()
                    });
                    self.fetch_sources(statement, params, mappers).await
                }
            },
            None => {
                let statement = self.own(self.readonly_statements.select(None), mappers, options)?;
                self.fetch_sources(&statement, parameters.unwrap_or(&[]).to_vec(), mappers)
                    .await
            }
        }
    }

    async fn fetch_sources(
        &self,
        statement: &dyn Statement,
        params: Vec<Parameter>,
        mappers: &StatementMappers,
    ) -> Result<Vec<Row>> {
        let reader = self
            .command()
            .execute_reader_statement(statement, Some(&params))
            .await?;
        let mut rows = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                rows.push(self.column_to_property_name(row, mappers));
            }
        }
        Ok(rows)
    }

    async fn apply(&self, pairs: Vec<(Row, Option<Row>)>) -> Result<Vec<(Row, Option<Row>)>> {
        if !self.enriched() {
            return Ok(pairs);
        }
        let (mut items, sources): (Vec<Row>, Vec<Option<Row>>) = pairs.into_iter().unzip();
        self.fill(&mut items, Some(&sources)).await?;
        Ok(match items.len() == sources.len() {
            true => items.into_iter().zip(sources).collect(),
            false => items.into_iter().map(|item| (item, None)).collect(),
        })
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
        match self.store_where(where_clause, false, mappers, options)? {
            Some(stored) => Ok((
                self.own(self.readonly_statements.select(Some(&stored)), mappers, options)?,
                stored.parameters(),
            )),
            None => Ok((
                self.own(
                    self.readonly_statements.select(Some(where_clause)),
                    mappers,
                    options,
                )?,
                where_clause.parameters(),
            )),
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
    async fn read_single(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Option<Row>> {
        Ok(self.read(query, parameters, options).await?.into_iter().next())
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
        options: Option<&ReadonlyRepositoryMappingOptions>,
    ) -> Result<Vec<Row>> {
        Ok(self
            .read_tracked(query, parameters, options)
            .await?
            .into_iter()
            .map(|(item, _)| item)
            .collect())
    }
}

pub(crate) fn push_middleware(
    repo: &ReadonlyRepository,
    middleware: Arc<dyn CommandMiddleware>,
) {
    repo.middleware_slot
        .lock()
        .expect("middleware")
        .push(middleware);
}
