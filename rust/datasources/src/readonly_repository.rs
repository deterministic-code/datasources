use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, MetaDataRowTypeConverter, RowTypeConverter, TypeConverter};
use crate::create_crud_repository_options::create_crud_repository_options;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::i_where::IWhere;
use crate::mappers::{IdentityMapper, Mapper};
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;
use crate::repository::IReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use crate::select_statement::SelectStatement;
use crate::shared_items::SharedItems;
use crate::statement::{Statement, StatementKind};
use crate::statements::Where;
use crate::statement_mappings::StatementMappings;
use crate::stored_enrichment::StoredEnrichment;
use crate::stubs::StubConnection;
use crate::stubs::StubRepository;
use crate::readonly_repository_builder::ReadonlyRepositoryBuilder;
use crate::readonly_repository_statements::ReadonlyRepositoryStatements;
use crate::value::Value;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::{Arc, Mutex};

pub struct ReadonlyRepository {
    pub(crate) connection: Arc<dyn Connection>,
    pub(crate) middlewares: Vec<Arc<dyn CommandMiddleware>>,
    pub(crate) row_converter: Option<RowTypeConverter>,
    pub(crate) object_converter: Box<dyn TypeConverter<Row, Row> + Send + Sync>,
    statement_mappings: StatementMappings,
    readonly_statements: ReadonlyRepositoryStatements,
    pub(crate) entity_type: TypeId,
    pub(crate) memory: Option<Arc<SharedItems>>,
    pub(crate) enrichments: Arc<Mutex<Vec<StoredEnrichment>>>,
    pub(crate) tree: RepositoryTree,
}

impl ReadonlyRepository {
    pub fn from_type<T: ConvertSchema + 'static>(connection: Arc<dyn Connection>) -> Result<Self> {
        let mut options = create_crud_repository_options::<T>();
        options.row_converter = Some(RowTypeConverter::of::<T>());
        let mut repo = Self::new(connection, options.read)?;
        repo.entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(
        connection: Arc<dyn Connection>,
        options: ReadonlyRepositoryOptions,
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
            row_converter: built.row_converter,
            object_converter: built.object_converter,
            statement_mappings: StatementMappings::new(built.entity_mapper, built.property_mappings),
            readonly_statements: built.statements,
            entity_type: TypeId::of::<Row>(),
            memory: None,
            tree: RepositoryTree::new(RepositoryTreeNode::new(
                Arc::new(StubRepository::new("readonly")),
                "",
                enrichments.clone(),
            )),
            enrichments,
        }
    }

    pub fn memory(items: Vec<Row>) -> Self {
        let enrichments = Arc::new(Mutex::new(Vec::new()));
        Self {
            connection: Arc::new(StubConnection::new("memory")),
            middlewares: Vec::new(),
            row_converter: None,
            object_converter: Box::new(MetaDataRowTypeConverter::new(Vec::new())),
            statement_mappings: {
                let entity: Arc<dyn Mapper<String, String> + Send + Sync> =
                    Arc::new(IdentityMapper::<String>::new());
                let property: Arc<dyn Mapper<String, String> + Send + Sync> =
                    Arc::new(IdentityMapper::<String>::new());
                StatementMappings::new(entity, property)
            },
            readonly_statements: ReadonlyRepositoryStatements::new(
                "",
                Vec::new(),
                StatementMappings::new(
                    Arc::new(IdentityMapper::<String>::new()),
                    Arc::new(IdentityMapper::<String>::new()),
                ),
            ),
            entity_type: TypeId::of::<Row>(),
            memory: Some(SharedItems::new(items)),
            tree: RepositoryTree::new(RepositoryTreeNode::new(
                Arc::new(StubRepository::new("memory")),
                "",
                enrichments.clone(),
            )),
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
        self.middlewares.push(middleware);
    }

    pub(crate) fn enriched(&self) -> bool {
        !self.enrichments.lock().expect("enrichments").is_empty()
    }

    pub(crate) fn memory_items(&self) -> Option<&Arc<SharedItems>> {
        self.memory.as_ref()
    }

    pub(crate) fn command(&self) -> Arc<dyn Command> {
        let command = self.connection.create_command();
        for middleware in &self.middlewares {
            command.add_middleware(middleware.clone());
        }
        command
    }

    fn query_of(&self, query: Option<&dyn Statement>) -> (SelectStatement, Vec<Parameter>) {
        if let Some(query) = query {
            let params = query
                .predicate()
                .map(|predicate| predicate.parameters())
                .unwrap_or_default();
            let statement = match query.predicate() {
                Some(predicate) => SelectStatement::with_where(
                    query.get_text(),
                    crate::where_text::WhereText::new(predicate.get_text()),
                ),
                None => SelectStatement::text(query.get_text()),
            };
            (statement, params)
        } else {
            (self.readonly_statements.select(None), Vec::new())
        }
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
    ) -> Result<Option<Where>> {
        match where_clause.as_any().downcast_ref::<Where>() {
            Some(clause) => Ok(Some(self.statement_mappings.apply_where(
                clause,
                &|name, cell| self.cell(name, cell, fields_applied),
            )?)),
            None => Ok(None),
        }
    }

    pub(crate) fn column(&self, property: &str) -> String {
        self.statement_mappings.column(property)
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
        match &self.row_converter {
            Some(converter) => match converter.fields.get(property) {
                Some(field) if !fields_applied => field.reverse_convert(value),
                _ => Ok(value.clone()),
            },
            None => Ok(value.clone()),
        }
    }

    async fn read_tracked(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Vec<(Row, Option<Row>)>> {
        self.apply(self.read_items(query, parameters).await?).await
    }

    async fn read_sources(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Vec<Row>> {
        if let Some(memory) = &self.memory {
            return Ok(memory.filter_input(query));
        }
        if let Some(where_clause) = query.and_then(ReadInput::as_where) {
            let (statement, params) = self.stored_query(where_clause)?;
            return self
                .fetch_sources(&statement, parameters.unwrap_or(&params).to_vec())
                .await;
        }
        let statement = query.and_then(ReadInput::as_statement);
        match statement {
            Some(statement) if statement.kind() == StatementKind::Procedure => {
                self.fetch_sources(statement, parameters.unwrap_or(&[]).to_vec())
                    .await
            }
            Some(statement) => match statement.as_any().downcast_ref::<SelectStatement>() {
                Some(select) if !select.table.is_empty() => {
                    let mapped = self.statement_mappings.apply_select(select, &|name, cell| {
                        self.cell(name, cell, false)
                    })?;
                    let params = parameters.map(|items| items.to_vec()).unwrap_or_else(|| {
                        mapped
                            .predicate()
                            .map(|predicate| predicate.parameters())
                            .unwrap_or_default()
                    });
                    self.fetch_sources(&mapped, params).await
                }
                _ => {
                    let (statement, params) = self.query_of(Some(statement));
                    self.fetch_sources(&statement, parameters.unwrap_or(&params).to_vec())
                        .await
                }
            },
            None => {
                let statement = self.own(self.readonly_statements.select(None))?;
                self.fetch_sources(&statement, parameters.unwrap_or(&[]).to_vec())
                    .await
            }
        }
    }

    async fn fetch_sources(
        &self,
        statement: &dyn Statement,
        params: Vec<Parameter>,
    ) -> Result<Vec<Row>> {
        let reader = self
            .command()
            .execute_reader_statement(statement, Some(&params))
            .await?;
        let mut rows = Vec::new();
        while reader.next().await? {
            if let Some(row) = reader.current() {
                rows.push(self.renamed(row));
            }
        }
        Ok(rows)
    }

    async fn read_items(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Vec<(Row, Option<Row>)>> {
        if self.memory.is_some() {
            return Ok(self
                .read_sources(query, parameters)
                .await?
                .into_iter()
                .map(|row| {
                    let source = row.clone();
                    (row, Some(source))
                })
                .collect());
        }
        let mut pairs = Vec::new();
        for renamed in self.read_sources(query, parameters).await? {
            let cells = match &self.row_converter {
                Some(converter) => converter.convert(&renamed)?,
                None => renamed.clone(),
            };
            let item = self.object_converter.convert(&cells)?;
            pairs.push((item, Some(renamed)));
        }
        Ok(pairs)
    }

    async fn apply(&self, pairs: Vec<(Row, Option<Row>)>) -> Result<Vec<(Row, Option<Row>)>> {
        if !self.enriched() {
            return Ok(pairs);
        }
        let mut items: Vec<Row> = pairs.iter().map(|(item, _)| item.clone()).collect();
        let sources: Vec<Option<Row>> = pairs.iter().map(|(_, source)| source.clone()).collect();
        self.fill(&mut items, Some(&sources)).await?;
        let paired = items.len() == sources.len();
        Ok(items
            .into_iter()
            .enumerate()
            .map(|(index, item)| (item, if paired { sources[index].clone() } else { None }))
            .collect())
    }

    fn renamed(&self, row: Row) -> Row {
        row.into_iter()
            .map(|(key, value)| (self.statement_mappings.property(&key), value))
            .collect()
    }

    fn own(&self, select: SelectStatement) -> Result<SelectStatement> {
        match select.table.is_empty() {
            true => Ok(select),
            false => self
                .statement_mappings
                .apply_select(&select, &|name, cell| self.cell(name, cell, false)),
        }
    }

    fn stored_query(&self, where_clause: &dyn IWhere) -> Result<(SelectStatement, Vec<Parameter>)> {
        match self.store_where(where_clause, false)? {
            Some(stored) => Ok((
                self.own(self.readonly_statements.select(Some(&stored)))?,
                stored.parameters(),
            )),
            None => Ok((
                self.own(self.readonly_statements.select(Some(where_clause)))?,
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
    ) -> Result<Option<Row>> {
        Ok(self.read(query, parameters).await?.into_iter().next())
    }

    async fn read(
        &self,
        query: Option<&dyn ReadInput>,
        parameters: Option<&[Parameter]>,
    ) -> Result<Vec<Row>> {
        Ok(self
            .read_tracked(query, parameters)
            .await?
            .into_iter()
            .map(|(item, _)| item)
            .collect())
    }
}
