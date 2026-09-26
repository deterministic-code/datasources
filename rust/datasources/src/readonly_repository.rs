use crate::built_repository::BuiltRepository;
use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_options::create_repository_options;
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
use crate::stored_enrichment::StoredEnrichment;
use crate::select_statement::SelectStatement;
use crate::shared_items::SharedItems;
use crate::statement::{Statement, StatementKind};
use crate::statements::Where;
use crate::stubs::StubConnection;
use crate::stubs::StubRepository;
use crate::table_repository_builder::RepositoryBuilder;
use crate::table_repository_statements::RepositoryStatements;
use crate::value::Value;
use async_trait::async_trait;
use std::any::TypeId;
use std::sync::{Arc, Mutex};

pub struct ReadonlyRepository {
    pub(crate) connection: Arc<dyn Connection>,
    pub(crate) middlewares: Vec<Arc<dyn CommandMiddleware>>,
    pub(crate) row_converter: RowTypeConverter,
    pub(crate) rows_mapper: Box<dyn Mapper<Row, Row> + Send + Sync>,
    pub(crate) column_mappings: Box<dyn Mapper<String, String> + Send + Sync>,
    pub(crate) statements: RepositoryStatements,
    pub(crate) entity_type: TypeId,
    pub(crate) memory: Option<Arc<SharedItems>>,
    pub(crate) enrichments: Arc<Mutex<Vec<StoredEnrichment>>>,
    pub(crate) tree: RepositoryTree,
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
        let enrichments = Arc::new(Mutex::new(Vec::new()));
        Self {
            connection: built.connection,
            middlewares: Vec::new(),
            row_converter: built.row_converter,
            rows_mapper: built.rows_mapper,
            column_mappings: built.column_mappings,
            statements: built.statements,
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
            row_converter: RowTypeConverter::new(Vec::new()),
            rows_mapper: Box::new(IdentityMapper::<Row>::new()),
            column_mappings: Box::new(IdentityMapper::<String>::new()),
            statements: RepositoryStatements::new("", Vec::new()),
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
        self.enrichments.lock().expect("enrichments").push(
            StoredEnrichment::direct(enrichment, via, None, strategy),
        );
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

    fn query_of(
        &self,
        query: Option<&dyn Statement>,
    ) -> (SelectStatement, Vec<Parameter>) {
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
            (self.statements.query(None), Vec::new())
        }
    }

    pub(crate) fn reverse_convert(&self, item: &Row) -> Result<Row> {
        self.row_converter
            .reverse_convert(&Mapper::reverse_map(&self.rows_mapper, item.clone()))
    }

    fn store_where(
        &self,
        where_clause: &dyn IWhere,
        fields_applied: bool,
    ) -> Result<Option<Where>> {
        match where_clause.as_any().downcast_ref::<Where>() {
            Some(clause) => Ok(Some(self.store_clause(clause, fields_applied)?)),
            None => Ok(None),
        }
    }

    fn store_clause(&self, clause: &Where, fields_applied: bool) -> Result<Where> {
        clause.map(
            |name| self.column(name),
            |name, cell| self.cell(name, cell, fields_applied),
        )
    }

    pub(crate) fn column(&self, property: &str) -> String {
        Mapper::map(&self.column_mappings, property.to_string())
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
        match self.row_converter.fields.get(property) {
            Some(field) if !fields_applied => field.reverse_convert(value),
            _ => Ok(value.clone()),
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
            statement => {
                let (statement, params) = self.query_of(statement);
                self.fetch_sources(&statement, parameters.unwrap_or(&params).to_vec())
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
            let item = Mapper::map(
                &self.rows_mapper,
                self.row_converter.convert(&renamed)?,
            );
            pairs.push((item, Some(renamed)));
        }
        Ok(pairs)
    }

    async fn apply(
        &self,
        pairs: Vec<(Row, Option<Row>)>,
    ) -> Result<Vec<(Row, Option<Row>)>> {
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
            .map(|(key, value)| (Mapper::reverse_map(&self.column_mappings, key), value))
            .collect()
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
