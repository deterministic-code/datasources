use crate::cascade_delete_enrichment_strategy::CascadeDeleteEnrichmentStrategy;
use crate::datasource_error::DatasourceError;
use crate::delete_enrichment_strategy::{load, DeleteEnrichmentStrategy};
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::{encode_rows, RepositoryTreeNode};
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::write_mode::WriteMode;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct DefaultWriteEnrichmentStrategy {
    delete: Arc<dyn DeleteEnrichmentStrategy>,
    skip_unloaded: bool,
}

impl DefaultWriteEnrichmentStrategy {
    pub fn new(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>) -> Self {
        Self {
            delete: delete.unwrap_or_else(|| Arc::new(CascadeDeleteEnrichmentStrategy)),
            skip_unloaded: false,
        }
    }

    pub fn lazy(delete: Option<Arc<dyn DeleteEnrichmentStrategy>>) -> Self {
        Self {
            delete: delete.unwrap_or_else(|| Arc::new(CascadeDeleteEnrichmentStrategy)),
            skip_unloaded: true,
        }
    }
}

impl Default for DefaultWriteEnrichmentStrategy {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl WriteEnrichmentStrategy for DefaultWriteEnrichmentStrategy {
    async fn persist(
        &self,
        tree: &RepositoryTree,
        items: &mut [Row],
        mode: WriteMode,
    ) -> Result<()> {
        self.walk(&tree.parent, items, mode).await
    }
}

impl DefaultWriteEnrichmentStrategy {
    fn walk<'a>(
        &'a self,
        node: &'a RepositoryTreeNode,
        parents: &'a mut [Row],
        mode: WriteMode,
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
        mode: WriteMode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let Some(enrichment) = child.enrichment.as_ref() else {
                return Ok(());
            };
            if enrichment.association.is_some() {
                let writer = child.link.as_ref().ok_or_else(|| {
                    DatasourceError::message(format!(
                        "Deep write requires a CRUD repository for \"{}\"",
                        enrichment.into
                    ))
                })?;
                for parent in parents.iter_mut() {
                    if self.skip(parent, &enrichment.into, mode) {
                        continue;
                    }
                    match mode {
                        WriteMode::Create => {
                            create_links(writer.as_ref(), enrichment, parent).await?
                        }
                        WriteMode::Update => {
                            sync_links(writer.as_ref(), enrichment, parent).await?
                        }
                        WriteMode::Delete => {
                            self.delete
                                .delete_linked(child, enrichment, parent)
                                .await?
                        }
                    }
                }
                return Ok(());
            }
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
                if self.skip(parent, &enrichment.into, mode) {
                    continue;
                }
                match mode {
                    WriteMode::Create => {
                        self.create_children(child, writer.as_ref(), enrichment, parent)
                            .await?
                    }
                    WriteMode::Update => {
                        self.sync_children(child, writer.as_ref(), enrichment, parent)
                            .await?
                    }
                    WriteMode::Delete => {
                        self.delete
                            .delete_owned(child, enrichment, parent, self)
                            .await?
                    }
                }
            }
            Ok(())
        })
    }

    fn skip(&self, parent: &Row, into: &str, mode: WriteMode) -> bool {
        self.skip_unloaded && mode != WriteMode::Delete && !parent.contains_key(into)
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
            copy_ids(row, &created, &writer.id_names());
            self.walk(node, std::slice::from_mut(row), WriteMode::Create)
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
        let keep: HashSet<String> = related
            .iter()
            .filter_map(|row| row_key(row, &ids))
            .collect();
        for row in existing.iter().filter(|row| {
            row_key(row, &ids)
                .map(|key| !keep.contains(&key))
                .unwrap_or(true)
        }) {
            let mut doomed = row.clone();
            self.walk(node, std::slice::from_mut(&mut doomed), WriteMode::Delete)
                .await?;
            writer.delete(row, None).await?;
        }
        for row in &mut related {
            match row_key(row, &ids) {
                Some(key)
                    if existing
                        .iter()
                        .any(|ex| row_key(ex, &ids) == Some(key.clone())) =>
                {
                    writer.update(&strip_enriched(row, node), None).await?;
                }
                _ => {
                    let mut stripped = strip_enriched(row, node);
                    let created = writer.create(&mut stripped).await?;
                    copy_ids(row, &created, &ids);
                }
            }
            self.walk(node, std::slice::from_mut(row), WriteMode::Update)
                .await?;
        }
        parent.insert(enrichment.into.clone(), encode_rows(&related));
        Ok(())
    }
}

#[async_trait]
impl crate::delete_nested::DeleteNested for DefaultWriteEnrichmentStrategy {
    async fn delete_nested(&self, node: &RepositoryTreeNode, rows: &mut [Row]) -> Result<()> {
        self.walk(node, rows, WriteMode::Delete).await
    }
}

async fn create_links(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
) -> Result<()> {
    let association = enrichment
        .association
        .as_ref()
        .ok_or_else(|| DatasourceError::message("Missing association"))?;
    let parent_key = required(parent, &enrichment.from, &enrichment.into)?;
    for id in related_ids(parent, enrichment)? {
        let mut row = Row::new();
        row.insert(association.parent.clone(), parent_key.clone());
        row.insert(association.related.clone(), id);
        writer.create(&mut row).await?;
    }
    Ok(())
}

async fn sync_links(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
) -> Result<()> {
    let association = enrichment
        .association
        .as_ref()
        .ok_or_else(|| DatasourceError::message("Missing association"))?;
    let parent_key = required(parent, &enrichment.from, &enrichment.into)?;
    let existing = writer
        .read(
            Some(&Where::new(Operator::equals(
                association.parent.clone(),
                parent_key.clone(),
            ))),
            None,
        )
        .await?;
    let wanted = related_ids(parent, enrichment)?;
    let keep: HashSet<String> = wanted.iter().map(Value::wire_text).collect();
    for row in &existing {
        let id = row.get(&association.related);
        if id.map(Value::wire_text).is_some_and(|key| keep.contains(&key)) {
            continue;
        }
        let clause = Where::new(Operator::and(
            Operator::equals(association.parent.clone(), parent_key.clone()),
            Operator::equals(
                association.related.clone(),
                id.cloned().unwrap_or(Value::Null),
            ),
        ));
        writer.delete(row, Some(&clause)).await?;
    }
    for id in wanted {
        if existing
            .iter()
            .any(|row| row.get(&association.related).map(Value::wire_text) == Some(id.wire_text()))
        {
            continue;
        }
        let mut row = Row::new();
        row.insert(association.parent.clone(), parent_key.clone());
        row.insert(association.related.clone(), id);
        writer.create(&mut row).await?;
    }
    Ok(())
}

fn related_ids(parent: &Row, enrichment: &Enrichment) -> Result<Vec<Value>> {
    let mut ids = Vec::new();
    let mut seen = HashSet::new();
    for related in decode_rows(parent.get(&enrichment.into)) {
        let id = related.get(&enrichment.by).cloned().ok_or_else(|| {
            DatasourceError::message(format!(
                "{} requires \"{}\"",
                enrichment.into, enrichment.by
            ))
        })?;
        if missing(&id) {
            return Err(DatasourceError::message(format!(
                "{} requires \"{}\"",
                enrichment.into, enrichment.by
            )));
        }
        if seen.insert(id.wire_text()) {
            ids.push(id);
        }
    }
    Ok(ids)
}

fn required(parent: &Row, name: &str, into: &str) -> Result<Value> {
    parent
        .get(name)
        .filter(|value| !missing(value))
        .cloned()
        .ok_or_else(|| DatasourceError::message(format!("{into} requires \"{name}\"")))
}

fn missing(value: &Value) -> bool {
    matches!(value, Value::Null | Value::Int(0))
        || matches!(value, Value::Text(text) if text.is_empty())
}

pub(crate) fn strip_enriched(item: &Row, node: &RepositoryTreeNode) -> Row {
    let mut row = item.clone();
    for child in &node.items {
        if let Some(enrichment) = &child.enrichment {
            row.remove(&enrichment.into);
        }
    }
    row
}

pub(crate) fn copy_ids(item: &mut Row, created: &Row, ids: &[String]) {
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
