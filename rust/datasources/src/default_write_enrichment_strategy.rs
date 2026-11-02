use crate::cascade_delete_enrichment_strategy::CascadeDeleteEnrichmentStrategy;
use crate::create_enrichment_strategy::CreateEnrichmentStrategy;
use crate::datasource_error::DatasourceError;
use crate::default_create_enrichment_strategy::DefaultCreateEnrichmentStrategy;
use crate::default_update_enrichment_strategy::DefaultUpdateEnrichmentStrategy;
use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository_tree::RepositoryTree;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::write_mode::WriteMode;
use async_trait::async_trait;
use std::collections::HashSet;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

pub struct DefaultWriteEnrichmentStrategy<T: Entity = Row> {
    create: Arc<dyn CreateEnrichmentStrategy<T>>,
    delete: Arc<dyn DeleteEnrichmentStrategy<T>>,
    update: Arc<dyn UpdateEnrichmentStrategy<T>>,
    skip_unloaded: bool,
    use_transaction: bool,
    entity: PhantomData<T>,
}

impl<T: Entity> DefaultWriteEnrichmentStrategy<T> {
    pub fn new(
        delete: Option<Arc<dyn DeleteEnrichmentStrategy<T>>>,
        update: Option<Arc<dyn UpdateEnrichmentStrategy<T>>>,
        create: Option<Arc<dyn CreateEnrichmentStrategy<T>>>,
        use_transaction: bool,
    ) -> Self {
        Self {
            create: create.unwrap_or_else(|| Arc::new(DefaultCreateEnrichmentStrategy)),
            delete: delete.unwrap_or_else(|| Arc::new(CascadeDeleteEnrichmentStrategy)),
            update: update.unwrap_or_else(|| Arc::new(DefaultUpdateEnrichmentStrategy)),
            skip_unloaded: false,
            use_transaction,
            entity: PhantomData,
        }
    }

    pub fn lazy(
        delete: Option<Arc<dyn DeleteEnrichmentStrategy<T>>>,
        update: Option<Arc<dyn UpdateEnrichmentStrategy<T>>>,
        create: Option<Arc<dyn CreateEnrichmentStrategy<T>>>,
        use_transaction: bool,
    ) -> Self {
        Self {
            skip_unloaded: true,
            ..Self::new(delete, update, create, use_transaction)
        }
    }
}

impl<T: Entity> Default for DefaultWriteEnrichmentStrategy<T> {
    fn default() -> Self {
        Self::new(None, None, None, false)
    }
}

#[async_trait]
impl<T: Entity> WriteEnrichmentStrategy<T> for DefaultWriteEnrichmentStrategy<T> {
    fn use_transaction(&self) -> bool {
        self.use_transaction
    }

    async fn resolve_lookups(&self, tree: &RepositoryTree<T>, items: &mut [T]) -> Result<()> {
        let entries = tree.parent.enrichments.lock().expect("enrichments").clone();
        crate::create_enrichment_strategy::resolve_lookups(&entries, items).await
    }

    async fn persist(
        &self,
        tree: &RepositoryTree<T>,
        items: &mut [T],
        mode: WriteMode,
    ) -> Result<()> {
        let entries = tree.parent.enrichments.lock().expect("enrichments").clone();
        self.walk(&entries, items, mode).await
    }

    async fn persist_bulk(
        &self,
        tree: &RepositoryTree<T>,
        items: &mut [T],
        mode: WriteMode,
    ) -> Result<()> {
        if mode == WriteMode::Delete {
            return self.persist(tree, items, mode).await;
        }
        let entries = tree.parent.enrichments.lock().expect("enrichments").clone();
        self.walk_bulk(&entries, items, mode).await
    }
}

impl<T: Entity> DefaultWriteEnrichmentStrategy<T> {
    fn walk<'a>(
        &'a self,
        entries: &'a [StoredEnrichment<T>],
        parents: &'a mut [T],
        mode: WriteMode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            for entry in entries {
                self.persist_entry(entry, parents, mode).await?;
            }
            Ok(())
        })
    }

    fn walk_bulk<'a>(
        &'a self,
        entries: &'a [StoredEnrichment<T>],
        parents: &'a mut [T],
        mode: WriteMode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            for entry in entries {
                self.persist_entry_bulk(entry, parents, mode).await?;
            }
            Ok(())
        })
    }

    fn persist_entry_bulk<'a>(
        &'a self,
        entry: &'a StoredEnrichment<T>,
        parents: &'a mut [T],
        mode: WriteMode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let enrichment = &entry.enrichment;
            if matches!(enrichment.r#type, crate::EnrichmentPropertyType::Custom) {
                return Err(DatasourceError::message(format!(
                    "Unsupported enrichment type \"{:?}\"",
                    enrichment.r#type
                )));
            }
            if enrichment.association.is_some() {
                if let Some(link) = &entry.link {
                    for parent in parents.iter_mut() {
                        if self.skip(parent, &enrichment.destination_property, mode) {
                            continue;
                        }
                        link.write(enrichment, parent, mode).await?;
                    }
                }
                return Ok(());
            }
            if enrichment.r#type == crate::EnrichmentPropertyType::List
                && enrichment.related_property.is_some()
            {
                return Ok(());
            }
            if enrichment.r#type != crate::EnrichmentPropertyType::List
                && enrichment.r#type != crate::EnrichmentPropertyType::Dictionary
            {
                return Ok(());
            }
            let mut children = Vec::new();
            for parent in parents.iter() {
                if self.skip(parent, &enrichment.destination_property, mode) {
                    continue;
                }
                children.extend(collect_children(parent, enrichment)?);
            }
            if children.is_empty() {
                return Ok(());
            }
            let persist = entry.persist.as_ref().ok_or_else(|| {
                DatasourceError::message(format!(
                    "Deep write requires a CRUD repository for \"{}\"",
                    enrichment.destination_property
                ))
            })?;
            if let Some(bulk) = persist.persist_bulk() {
                return match mode {
                    WriteMode::Create => bulk.create_bulk(&mut children).await.map(|_| ()),
                    WriteMode::Update => bulk.update_bulk(&children, None).await.map(|_| ()),
                    WriteMode::Delete => Ok(()),
                };
            }
            for parent in parents.iter_mut() {
                if self.skip(parent, &enrichment.destination_property, mode) {
                    continue;
                }
                persist.write(enrichment, parent, mode).await?;
            }
            Ok(())
        })
    }

    fn persist_entry<'a>(
        &'a self,
        entry: &'a StoredEnrichment<T>,
        parents: &'a mut [T],
        mode: WriteMode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let enrichment = &entry.enrichment;
            if matches!(enrichment.r#type, crate::EnrichmentPropertyType::Custom) {
                return Err(DatasourceError::message(format!(
                    "Unsupported enrichment type \"{:?}\"",
                    enrichment.r#type
                )));
            }
            if enrichment.association.is_some() {
                let link = entry.link.as_ref().ok_or_else(|| {
                    DatasourceError::message(format!(
                        "Deep write requires a CRUD repository for \"{}\"",
                        enrichment.destination_property
                    ))
                })?;
                for parent in parents.iter_mut() {
                    if self.skip(parent, &enrichment.destination_property, mode) {
                        continue;
                    }
                    if let Some(writer) = link.row_writer() {
                        match mode {
                            WriteMode::Create => {
                                self.create.create_linked(writer, enrichment, parent).await?
                            }
                            WriteMode::Update => {
                                self.update.update_linked(writer, enrichment, parent).await?
                            }
                            WriteMode::Delete => {
                                self.delete.delete_linked(writer, enrichment, parent).await?
                            }
                        }
                    } else {
                        link.write(enrichment, parent, mode).await?;
                    }
                }
                return Ok(());
            }
            if enrichment.r#type == crate::EnrichmentPropertyType::List
                && enrichment.related_property.is_some()
            {
                return Ok(());
            }
            if enrichment.r#type != crate::EnrichmentPropertyType::List
                && enrichment.r#type != crate::EnrichmentPropertyType::Dictionary
            {
                return Ok(());
            }
            let persist = entry.persist.as_ref().ok_or_else(|| {
                DatasourceError::message(format!(
                    "Deep write requires a CRUD repository for \"{}\"",
                    enrichment.destination_property
                ))
            })?;
            for parent in parents.iter_mut() {
                if self.skip(parent, &enrichment.destination_property, mode) {
                    continue;
                }
                if let Some(writer) = persist.row_writer() {
                    match mode {
                        WriteMode::Create => {
                            self.create.create_children(writer, enrichment, parent).await?
                        }
                        WriteMode::Update => {
                            self.update.update_children(writer, enrichment, parent).await?
                        }
                        WriteMode::Delete => {
                            self.delete
                                .delete_children(writer, enrichment, parent, self)
                                .await?
                        }
                    }
                } else {
                    persist.write(enrichment, parent, mode).await?;
                }
            }
            Ok(())
        })
    }

    fn skip(&self, parent: &T, into: &str, mode: WriteMode) -> bool {
        self.skip_unloaded && mode != WriteMode::Delete && !parent.has(into)
    }
}

#[async_trait]
impl<T: Entity> crate::delete_nested::DeleteNested for DefaultWriteEnrichmentStrategy<T> {
    async fn delete_nested(&self, _rows: &mut [Row]) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn related_ids<T: Entity>(parent: &T, enrichment: &Enrichment) -> Result<Vec<Value>> {
    let mut ids = Vec::new();
    let mut seen = HashSet::new();
    for related in parent.payload(&enrichment.destination_property) {
        let id = related
            .get(&enrichment.related_key)
            .cloned()
            .ok_or_else(|| {
                DatasourceError::message(format!(
                    "{} requires \"{}\"",
                    enrichment.destination_property, enrichment.related_key
                ))
            })?;
        if missing(&id) {
            return Err(DatasourceError::message(format!(
                "{} requires \"{}\"",
                enrichment.destination_property, enrichment.related_key
            )));
        }
        if seen.insert(id.wire_text()) {
            ids.push(id);
        }
    }
    Ok(ids)
}

pub(crate) fn collect_children<T: Entity>(parent: &T, enrichment: &Enrichment) -> Result<Vec<Row>> {
    let mut related = parent.payload(&enrichment.destination_property);
    if related.is_empty() {
        return Ok(related);
    }
    let key = required(
        parent,
        &enrichment.source_property,
        &enrichment.destination_property,
    )?;
    for row in &mut related {
        row.insert(enrichment.related_key.clone(), key.clone());
    }
    Ok(related)
}

pub(crate) fn required<T: Entity>(parent: &T, name: &str, into: &str) -> Result<Value> {
    parent
        .cell(name)
        .filter(|value| !missing(value))
        .ok_or_else(|| DatasourceError::message(format!("{into} requires \"{name}\"")))
}

fn missing(value: &Value) -> bool {
    matches!(value, Value::Null | Value::Int(0))
        || matches!(value, Value::Text(text) if text.is_empty())
}

#[allow(dead_code)]
fn assigned(value: &Value) -> bool {
    match value {
        Value::Null | Value::Int(0) => false,
        Value::Text(text) if text.is_empty() => false,
        _ => true,
    }
}

#[allow(dead_code)]
pub(crate) fn row_key(row: &Row, ids: &[String]) -> Option<String> {
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

pub fn decode_rows(value: Option<&Value>) -> Vec<Row> {
    match value {
        Some(Value::Text(text)) if !text.is_empty() => text.split('|').map(decode_row).collect(),
        _ => Vec::new(),
    }
}

fn decode_row(text: &str) -> Row {
    text.split(',')
        .filter_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            Some((key.to_string(), parse_value(&unescape(value))))
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

pub fn encode_rows(rows: &[Row]) -> Value {
    Value::Text(
        rows.iter()
            .map(encode_row_text)
            .collect::<Vec<_>>()
            .join("|"),
    )
}

#[allow(dead_code)]
pub(crate) fn encode_row(row: &Row) -> Value {
    Value::Text(encode_row_text(row))
}

fn unescape(text: &str) -> String {
    text.replace("%7C", "|")
        .replace("%2C", ",")
        .replace("%3D", "=")
        .replace("%25", "%")
}

fn escape_cell(text: &str) -> String {
    text.replace('%', "%25")
        .replace('|', "%7C")
        .replace(',', "%2C")
        .replace('=', "%3D")
}

fn encode_row_text(row: &Row) -> String {
    let mut keys: Vec<_> = row.keys().cloned().collect();
    keys.sort();
    keys.into_iter()
        .filter_map(|key| {
            row.get(&key)
                .map(|value| format!("{key}={}", escape_cell(&value.wire_text())))
        })
        .collect::<Vec<_>>()
        .join(",")
}
