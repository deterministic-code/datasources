use crate::built_repository::BuiltRepository;
use crate::command_middleware::CommandMiddleware;
use crate::connection::Connection;
use crate::converters::{ConvertSchema, RowTypeConverter};
use crate::create_repository_options::create_repository_options;
use crate::crud_repository_options::{identity_mapper, CRUDRepositoryOptions};
use crate::datasource_error::DatasourceError;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enriched::Enriched;
use crate::enrichment::Enrichment;
use crate::enrichment_via::EnrichmentVia;
use crate::i_repository::IRepository;
use crate::id_key::IdKey;
use crate::mappers::{Mapper, TransformMapper};
use crate::nest::Nest;
use crate::parameter::Parameter;
use crate::read_input::ReadInput;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::readonly_repository::ReadonlyRepository;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::{encode_rows, RepositoryTreeNode};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::statements::Text;
use crate::statement::Statement;
use crate::stubs::StubRepository;
use crate::table_repository_builder::RepositoryBuilder;
use crate::value::Value;
use crate::where_trait::Where;
use crate::{Expression, Literal, Operator, WhereClause};
use async_trait::async_trait;
use std::any::TypeId;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone, Copy)]
enum Mode {
    Create,
    Update,
    Delete,
}

pub struct CRUDRepository {
    inner: Option<ReadonlyRepository>,
    create: TransformMapper<Value, Row>,
    ids: Vec<String>,
    delegate: Option<Arc<dyn ICRUDRepository>>,
    pub tree: RepositoryTree,
    strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
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
        repo.inner
            .as_mut()
            .expect("sql repository")
            .entity_type = TypeId::of::<T>();
        Ok(repo)
    }

    pub fn new(connection: Arc<dyn Connection>, options: CRUDRepositoryOptions) -> Result<Self> {
        Ok(Self::from_built(RepositoryBuilder::parts_crud(
            connection, options,
        )?))
    }

    pub fn from_repository(inner: Arc<dyn ICRUDRepository>) -> Self {
        let strategy: Arc<dyn ReadonlyEnrichmentStrategy> =
            Arc::new(DefaultReadonlyEnrichmentStrategy);
        let id = inner.id_names();
        Self {
            inner: None,
            create: identity_mapper(&id),
            ids: id,
            tree: RepositoryTree::new(RepositoryTreeNode::with_strategy(
                as_readonly(inner.clone()),
                "",
                strategy.clone(),
            )),
            strategy,
            delegate: Some(inner),
        }
    }

    pub fn from_built(mut built: BuiltRepository) -> Self {
        let id = built.ids.clone();
        let create = built
            .create
            .take()
            .unwrap_or_else(|| identity_mapper(&id));
        let strategy: Arc<dyn ReadonlyEnrichmentStrategy> =
            Arc::new(DefaultReadonlyEnrichmentStrategy);
        Self {
            inner: Some(ReadonlyRepository::from_built(built)),
            create,
            ids: id,
            delegate: None,
            tree: RepositoryTree::new(RepositoryTreeNode::with_strategy(
                Arc::new(StubRepository::new("crud")),
                "",
                strategy.clone(),
            )),
            strategy,
        }
    }

    pub fn add_enriched(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: impl EnrichmentVia,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl Nest,
    ) -> &mut Self {
        nest.apply(
            self.tree
                .parent
                .include(via, into, from, by, select, many),
        );
        self
    }

    fn sql(&self) -> &ReadonlyRepository {
        self.inner.as_ref().expect("sql repository")
    }

    fn sql_mut(&mut self) -> &mut ReadonlyRepository {
        self.inner.as_mut().expect("sql repository")
    }

    fn enriched(&self) -> bool {
        !self.tree.parent.items.is_empty()
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn CommandMiddleware>) {
        self.sql_mut().add_middleware(middleware);
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
                    self.sql().column(&name),
                    self.sql().cell(&name, &value, true)?,
                ))
            })
            .collect()
    }

    fn id_where(&self, record: &Row) -> Result<WhereClause> {
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
        Ok(WhereClause::new(and_all(children)))
    }
}

impl IRepository for CRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.delegate
            .as_ref()
            .map(|repo| repo.entity_type())
            .unwrap_or_else(|| self.sql().entity_type())
    }
}

#[async_trait]
impl Enriched for CRUDRepository {
    async fn enrich(&self, items: &mut [Row]) -> Result<()> {
        self.strategy.enrich(&self.tree, items).await
    }
}

#[async_trait]
impl IReadonlyRepository for CRUDRepository {
    fn to_crud(self: Arc<Self>) -> Option<Arc<dyn ICRUDRepository>> {
        Some(self)
    }

    async fn read_single(&self, query: Option<&dyn ReadInput>) -> Result<Option<Row>> {
        if self.enriched() {
            return Ok(self.read(query).await?.into_iter().next());
        }
        match &self.delegate {
            Some(repo) => repo.read_single(query).await,
            None => self.sql().read_single(query).await,
        }
    }

    async fn read(&self, query: Option<&dyn ReadInput>) -> Result<Vec<Row>> {
        let mut rows = match &self.delegate {
            Some(repo) => repo.read(query).await?,
            None => self.sql().read(query).await?,
        };
        if self.enriched() {
            self.strategy.enrich(&self.tree, &mut rows).await?;
        }
        Ok(rows)
    }
}

#[async_trait]
impl ICRUDRepository for CRUDRepository {
    async fn create(&self, item: &mut Row) -> Result<Row> {
        if !self.enriched() {
            return match &self.delegate {
                Some(repo) => repo.create(item).await,
                None => self.inserted(item).await,
            };
        }
        let mut stripped = strip_enriched(item, &self.tree.parent);
        let created = match &self.delegate {
            Some(repo) => repo.create(&mut stripped).await?,
            None => self.inserted(&mut stripped).await?,
        };
        let ids = self.id_names();
        copy_ids(item, &created, &ids);
        self.persist(&self.tree.parent, std::slice::from_mut(item), Mode::Create)
            .await?;
        if ids.len() != 1 {
            return Ok(item.clone());
        }
        let Some(id) = item.get(&ids[0]).cloned() else {
            return Ok(item.clone());
        };
        if matches!(id, Value::Null) {
            return Ok(item.clone());
        }
        Ok(self.read_single_by_id(&id).await?.unwrap_or_else(|| item.clone()))
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        if !self.enriched() {
            return match &self.delegate {
                Some(repo) => repo.update(item, by_where).await,
                None => self.save(item, by_where).await,
            };
        }
        let mut parent = item.clone();
        let response = match &self.delegate {
            Some(repo) => {
                repo.update(&strip_enriched(&parent, &self.tree.parent), by_where)
                    .await?
            }
            None => {
                self.save(&strip_enriched(&parent, &self.tree.parent), by_where)
                    .await?
            }
        };
        self.persist(
            &self.tree.parent,
            std::slice::from_mut(&mut parent),
            Mode::Update,
        )
        .await?;
        Ok(response)
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        if self.enriched() {
            let mut parent = item.clone();
            self.persist(
                &self.tree.parent,
                std::slice::from_mut(&mut parent),
                Mode::Delete,
            )
            .await?;
        }
        match &self.delegate {
            Some(repo) => repo.delete(item, by_where).await,
            None => self.remove(item, by_where).await,
        }
    }

    fn id_names(&self) -> Vec<String> {
        self.ids.clone()
    }

    async fn read_single_by_id(&self, id: &dyn IdKey) -> Result<Option<Row>> {
        if let Some(repo) = &self.delegate {
            return match repo.read_single_by_id(id).await? {
                Some(mut row) if self.enriched() => {
                    self.enrich(std::slice::from_mut(&mut row)).await?;
                    Ok(Some(row))
                }
                other => Ok(other),
            };
        }
        let record = self.id_record(id)?;
        let clause = self.id_where(&record)?;
        self.read_single(Some(&clause)).await
    }
}

impl CRUDRepository {
    async fn inserted(&self, item: &mut Row) -> Result<Row> {
        self.insert(item).await?;
        Ok(item.clone())
    }

    async fn insert(&self, item: &mut Row) -> Result<Response> {
        let record = self.sql().reverse_convert(item)?;
        let fields = self.params(&record, false)?;
        let statement = self.sql().statements.insert(&fields);
        let response = self
            .sql()
            .command()
            .execute_statement(&statement, Some(&fields))
            .await?;
        if response.result != Value::Null {
            apply(item, Mapper::map(&self.create, response.result.clone()));
        }
        Ok(response)
    }

    async fn save(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.sql().reverse_convert(item)?;
        let fields = self.params(&record, false)?;
        if fields.is_empty() {
            return Err(DatasourceError::message(
                "Update requires at least one non-Id field",
            ));
        }
        let (owned, raw) = self.stored_predicate(&record, by_where)?;
        let predicate: &dyn Where = match (owned.as_ref(), raw) {
            (Some(clause), _) => clause,
            (None, Some(clause)) => clause,
            _ => unreachable!(),
        };
        let statement = self.sql().statements.update(&fields, predicate);
        let parameters = statement.parameters();
        self.sql()
            .command()
            .execute(&statement.get_text(), Some(&parameters))
            .await
    }

    async fn remove(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let record = self.sql().reverse_convert(item)?;
        let (owned, raw) = self.stored_predicate(&record, by_where)?;
        let predicate: &dyn Where = match (owned.as_ref(), raw) {
            (Some(clause), _) => clause,
            (None, Some(clause)) => clause,
            _ => unreachable!(),
        };
        let statement = self.sql().statements.delete(predicate);
        let parameters = predicate.parameters();
        let parameters = if parameters.is_empty() && statement.get_text().contains('$') {
            self.params(&record, true)?
        } else {
            parameters
        };
        self.sql()
            .command()
            .execute_statement(&statement, Some(&parameters))
            .await
    }

    fn stored_predicate<'a>(
        &'a self,
        record: &Row,
        by_where: Option<&'a dyn Where>,
    ) -> Result<(Option<WhereClause>, Option<&'a dyn Where>)> {
        match by_where {
            Some(clause) => match self.sql().store_where(clause, false)? {
                Some(stored) => Ok((Some(stored), None)),
                None => Ok((None, Some(clause))),
            },
            None => Ok((
                Some(self.sql().store_clause(&self.id_where(record)?, true)?),
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

    fn persist<'a>(
        &'a self,
        node: &'a RepositoryTreeNode,
        parents: &'a mut [Row],
        mode: Mode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            for child in &node.items {
                self.persist_node(child, parents, mode).await?;
            }
            Ok(())
        })
    }

    fn persist_node<'a>(
        &'a self,
        child: &'a RepositoryTreeNode,
        parents: &'a mut [Row],
        mode: Mode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let Some(enrichment) = child.enrichment.as_ref() else {
                return Ok(());
            };
            if !enrichment.many || enrichment.select.is_some() {
                return Ok(());
            }
            let writer = child.write.as_ref().ok_or_else(|| {
                DatasourceError::message(format!(
                    "Deep write requires a CRUD repository for \"{}\"",
                    enrichment.into
                ))
            })?;
            for parent in parents.iter_mut() {
                match mode {
                    Mode::Create => {
                        self.create_children(child, writer.as_ref(), enrichment, parent)
                            .await?
                    }
                    Mode::Update => {
                        self.sync_children(child, writer.as_ref(), enrichment, parent)
                            .await?
                    }
                    Mode::Delete => {
                        self.delete_children(child, writer.as_ref(), enrichment, parent)
                            .await?
                    }
                }
            }
            Ok(())
        })
    }

    async fn create_children(
        &self,
        node: &RepositoryTreeNode,
        writer: &dyn ICRUDRepository,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        let mut related = decode_rows(parent.get(&enrichment.into));
        let key = parent.get(&enrichment.from).cloned();
        for row in &mut related {
            if let Some(key) = &key {
                row.insert(enrichment.by.clone(), key.clone());
            }
            let mut stripped = strip_enriched(row, node);
            let created = writer.create(&mut stripped).await?;
            let ids = writer.id_names();
            copy_ids(row, &created, &ids);
            self.persist(node, std::slice::from_mut(row), Mode::Create)
                .await?;
        }
        parent.insert(enrichment.into.clone(), encode_rows(&related));
        Ok(())
    }

    async fn sync_children(
        &self,
        node: &RepositoryTreeNode,
        writer: &dyn ICRUDRepository,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        let existing = load(writer, enrichment, parent).await?;
        let mut related = decode_rows(parent.get(&enrichment.into));
        if let Some(key) = parent.get(&enrichment.from).cloned() {
            for row in &mut related {
                row.insert(enrichment.by.clone(), key.clone());
            }
        }
        let ids = writer.id_names();
        let keep: HashSet<String> = related.iter().filter_map(|row| row_key(row, &ids)).collect();
        for row in existing
            .iter()
            .filter(|row| row_key(row, &ids).map(|key| !keep.contains(&key)).unwrap_or(true))
        {
            let mut doomed = row.clone();
            self.persist(node, std::slice::from_mut(&mut doomed), Mode::Delete)
                .await?;
            writer.delete(row, None).await?;
        }
        for row in &mut related {
            match row_key(row, &ids) {
                Some(key) if existing.iter().any(|ex| row_key(ex, &ids) == Some(key.clone())) => {
                    writer.update(&strip_enriched(row, node), None).await?;
                }
                _ => {
                    let mut stripped = strip_enriched(row, node);
                    let created = writer.create(&mut stripped).await?;
                    copy_ids(row, &created, &ids);
                }
            }
            self.persist(node, std::slice::from_mut(row), Mode::Update)
                .await?;
        }
        parent.insert(enrichment.into.clone(), encode_rows(&related));
        Ok(())
    }

    async fn delete_children(
        &self,
        node: &RepositoryTreeNode,
        writer: &dyn ICRUDRepository,
        enrichment: &Enrichment,
        parent: &Row,
    ) -> Result<()> {
        let mut existing = load(writer, enrichment, parent).await?;
        self.persist(node, &mut existing, Mode::Delete).await?;
        for row in existing {
            writer.delete(&row, None).await?;
        }
        Ok(())
    }
}

fn apply(item: &mut Row, identity: Row) {
    for (key, value) in identity {
        if value != Value::Null {
            item.insert(key, value);
        }
    }
}

fn strip_enriched(item: &Row, node: &RepositoryTreeNode) -> Row {
    let mut row = item.clone();
    for child in &node.items {
        if let Some(enrichment) = &child.enrichment {
            row.remove(&enrichment.into);
        }
    }
    row
}

fn copy_ids(item: &mut Row, created: &Row, ids: &[String]) {
    for name in ids {
        if let Some(value) = created.get(name) {
            if *value != Value::Null && *value != Value::Int(0) {
                item.insert(name.clone(), value.clone());
            }
        }
    }
}

fn assigned(value: &Value) -> bool {
    match value {
        Value::Null | Value::Int(0) => false,
        Value::Text(text) if text.is_empty() => false,
        _ => true,
    }
}

fn row_key(row: &Row, ids: &[String]) -> Option<String> {
    if ids.is_empty()
        || ids.iter().any(|name| match row.get(name) {
            Some(value) => !assigned(value),
            None => true,
        })
    {
        return None;
    }
    Some(
        ids.iter()
            .map(|name| row.get(name).map(Value::wire_text).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\u{1f}"),
    )
}

async fn load(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
) -> Result<Vec<Row>> {
    let Some(key) = parent.get(&enrichment.from).cloned() else {
        return Ok(Vec::new());
    };
    writer
        .read(Some(&WhereClause::new(Operator::equals(
            enrichment.by.clone(),
            key,
        ))))
        .await
}

fn decode_rows(value: Option<&Value>) -> Vec<Row> {
    match value {
        Some(Value::Text(text)) if !text.is_empty() => text.split('|').map(decode_row).collect(),
        _ => Vec::new(),
    }
}

fn decode_row(text: &str) -> Row {
    text.split(',')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((key.to_string(), parse_value(value)))
        })
        .collect()
}

fn parse_value(text: &str) -> Value {
    match text {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        _ => text
            .parse::<i64>()
            .map(Value::Int)
            .unwrap_or_else(|_| Value::Text(text.into())),
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
