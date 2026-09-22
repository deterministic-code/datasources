use crate::{Operator, WhereClause};
use crate::datasource_error::DatasourceError;
use crate::default_readonly_enrichment_strategy::DefaultReadonlyEnrichmentStrategy;
use crate::enriched::Enriched;
use crate::enriched_readonly_repository::EnrichedReadonlyRepository;
use crate::enrichment::Enrichment;
use crate::query::Query;
use crate::readonly_enrichment_strategy::ReadonlyEnrichmentStrategy;
use crate::i_repository::IRepository;
use crate::repository::{as_readonly, ICRUDRepository, IReadonlyRepository};
use crate::repository_tree_node::{encode_rows, RepositoryTreeNode};
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::where_trait::Where;
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

pub struct EnrichedCRUDRepository {
    inner: Arc<dyn ICRUDRepository>,
    enriched: EnrichedReadonlyRepository,
}

impl EnrichedCRUDRepository {
    pub fn new(inner: Arc<dyn ICRUDRepository>) -> Self {
        Self::with_strategy(inner, Arc::new(DefaultReadonlyEnrichmentStrategy))
    }

    pub fn with_strategy(
        inner: Arc<dyn ICRUDRepository>,
        strategy: Arc<dyn ReadonlyEnrichmentStrategy>,
    ) -> Self {
        Self {
            enriched: EnrichedReadonlyRepository::with_strategy(
                as_readonly(inner.clone()),
                strategy,
            ),
            inner,
        }
    }

    pub fn add_enriched(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn IReadonlyRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        self.enriched
            .add_enriched(into, from, via, by, select, many);
        self
    }

    pub fn add_enriched_with(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn IReadonlyRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl FnOnce(&mut RepositoryTreeNode),
    ) -> &mut Self {
        self.enriched
            .add_enriched_with(into, from, via, by, select, many, nest);
        self
    }

    pub fn add_enriched_repo(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn ICRUDRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
    ) -> &mut Self {
        self.enriched
            .add_enriched_repo(into, from, via, by, select, many);
        self
    }

    pub fn add_enriched_repo_with(
        &mut self,
        into: impl Into<String>,
        from: impl Into<String>,
        via: Arc<dyn ICRUDRepository>,
        by: impl Into<String>,
        select: Option<String>,
        many: bool,
        nest: impl FnOnce(&mut RepositoryTreeNode),
    ) -> &mut Self {
        self.enriched
            .add_enriched_repo_with(into, from, via, by, select, many, nest);
        self
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
            let response = writer.create(&strip_enriched(row, node)).await?;
            apply_identity(row, &response);
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
        let keep: HashSet<String> = related
            .iter()
            .filter_map(row_id)
            .filter(has_id)
            .map(id_key)
            .collect();
        for row in existing.iter().filter(|row| {
            row_id(row)
                .map(|id| !keep.contains(&id_key(id)))
                .unwrap_or(true)
        }) {
            let mut doomed = row.clone();
            self.persist(node, std::slice::from_mut(&mut doomed), Mode::Delete)
                .await?;
            writer.delete(row, None).await?;
        }
        for row in &mut related {
            match row_id(row) {
                Some(id)
                    if has_id(&id)
                        && existing.iter().any(|ex| {
                            row_id(ex).map(|other| id_key(other) == id_key(id.clone()))
                                == Some(true)
                        }) =>
                {
                    writer.update(&strip_enriched(row, node), None).await?;
                }
                _ => {
                    let response = writer.create(&strip_enriched(row, node)).await?;
                    apply_identity(row, &response);
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

fn strip_enriched(item: &Row, node: &RepositoryTreeNode) -> Row {
    let mut row = item.clone();
    for child in &node.items {
        if let Some(enrichment) = &child.enrichment {
            row.remove(&enrichment.into);
        }
    }
    row
}

fn apply_identity(item: &mut Row, response: &Response) {
    if response.result == Value::Null {
        return;
    }
    let name = if item.contains_key("id") || !item.contains_key("Id") {
        "id"
    } else {
        "Id"
    };
    item.insert(name.into(), response.result.clone());
}

fn row_id(row: &Row) -> Option<Value> {
    row.get("id").cloned().or_else(|| row.get("Id").cloned())
}

fn has_id(id: &Value) -> bool {
    match id {
        Value::Null | Value::Int(0) => false,
        Value::Text(text) if text.is_empty() => false,
        _ => true,
    }
}

fn id_key(id: Value) -> String {
    id.wire_text()
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
        .read_where(&WhereClause::new(Operator::equals(enrichment.by.clone(), key)))
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

#[async_trait]
impl Enriched for EnrichedCRUDRepository {
    async fn enrich(&self, items: &mut [Row]) -> Result<()> {
        self.enriched.enrich(items).await
    }
}

impl IRepository for EnrichedCRUDRepository {
    fn entity_type(&self) -> TypeId {
        self.inner.entity_type()
    }
}

#[async_trait]
impl IReadonlyRepository for EnrichedCRUDRepository {
    async fn read_single(&self, query: Option<&dyn Query>) -> Result<Option<Row>> {
        self.enriched.read_single(query).await
    }

    async fn read(&self, query: Option<&dyn Query>) -> Result<Vec<Row>> {
        self.enriched.read(query).await
    }

    async fn read_single_where(&self, where_clause: &dyn Where) -> Result<Option<Row>> {
        self.enriched.read_single_where(where_clause).await
    }

    async fn read_where(&self, where_clause: &dyn Where) -> Result<Vec<Row>> {
        self.enriched.read_where(where_clause).await
    }
}

#[async_trait]
impl ICRUDRepository for EnrichedCRUDRepository {
    async fn create(&self, item: &Row) -> Result<Response> {
        let mut parent = item.clone();
        let response = self
            .inner
            .create(&strip_enriched(&parent, &self.enriched.tree.parent))
            .await?;
        apply_identity(&mut parent, &response);
        self.persist(
            &self.enriched.tree.parent,
            std::slice::from_mut(&mut parent),
            Mode::Create,
        )
        .await?;
        Ok(response)
    }

    async fn update(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let mut parent = item.clone();
        let response = self
            .inner
            .update(
                &strip_enriched(&parent, &self.enriched.tree.parent),
                by_where,
            )
            .await?;
        self.persist(
            &self.enriched.tree.parent,
            std::slice::from_mut(&mut parent),
            Mode::Update,
        )
        .await?;
        Ok(response)
    }

    async fn delete(&self, item: &Row, by_where: Option<&dyn Where>) -> Result<Response> {
        let mut parent = item.clone();
        self.persist(
            &self.enriched.tree.parent,
            std::slice::from_mut(&mut parent),
            Mode::Delete,
        )
        .await?;
        self.inner.delete(item, by_where).await
    }
}
