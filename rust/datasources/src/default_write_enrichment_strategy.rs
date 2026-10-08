use crate::cascade_delete_enrichment_strategy::CascadeDeleteEnrichmentStrategy;
use crate::create_enrichment_strategy::CreateEnrichmentStrategy;
use crate::datasource_error::DatasourceError;
use crate::default_create_enrichment_strategy::DefaultCreateEnrichmentStrategy;
use crate::default_update_enrichment_strategy::DefaultUpdateEnrichmentStrategy;
use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use crate::enrichment::Enrichment;
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
use std::pin::Pin;
use std::sync::Arc;

pub struct DefaultWriteEnrichmentStrategy {
    create: Arc<dyn CreateEnrichmentStrategy>,
    delete: Arc<dyn DeleteEnrichmentStrategy>,
    update: Arc<dyn UpdateEnrichmentStrategy>,
    skip_unloaded: bool,
    use_transaction: bool,
}

impl DefaultWriteEnrichmentStrategy {
    pub fn new(
        delete: Option<Arc<dyn DeleteEnrichmentStrategy>>,
        update: Option<Arc<dyn UpdateEnrichmentStrategy>>,
        create: Option<Arc<dyn CreateEnrichmentStrategy>>,
        use_transaction: bool,
    ) -> Self {
        Self {
            create: create.unwrap_or_else(|| Arc::new(DefaultCreateEnrichmentStrategy)),
            delete: delete.unwrap_or_else(|| Arc::new(CascadeDeleteEnrichmentStrategy)),
            update: update.unwrap_or_else(|| Arc::new(DefaultUpdateEnrichmentStrategy)),
            skip_unloaded: false,
            use_transaction,
        }
    }

    pub fn lazy(
        delete: Option<Arc<dyn DeleteEnrichmentStrategy>>,
        update: Option<Arc<dyn UpdateEnrichmentStrategy>>,
        create: Option<Arc<dyn CreateEnrichmentStrategy>>,
        use_transaction: bool,
    ) -> Self {
        Self {
            skip_unloaded: true,
            ..Self::new(delete, update, create, use_transaction)
        }
    }
}

impl Default for DefaultWriteEnrichmentStrategy {
    fn default() -> Self {
        Self::new(None, None, None, false)
    }
}

#[async_trait]
impl WriteEnrichmentStrategy for DefaultWriteEnrichmentStrategy {
    fn use_transaction(&self) -> bool {
        self.use_transaction
    }

    async fn persist(
        &self,
        tree: &RepositoryTree,
        items: &mut [Row],
        mode: WriteMode,
    ) -> Result<()> {
        let entries = tree.parent.enrichments.lock().expect("enrichments").clone();
        self.walk(&entries, items, mode).await
    }
}

impl DefaultWriteEnrichmentStrategy {
    fn walk<'a>(
        &'a self,
        entries: &'a [StoredEnrichment],
        parents: &'a mut [Row],
        mode: WriteMode,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            for entry in entries {
                self.persist_entry(entry, parents, mode).await?;
            }
            Ok(())
        })
    }

    fn persist_entry<'a>(
        &'a self,
        entry: &'a StoredEnrichment,
        parents: &'a mut [Row],
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
                let writer = entry.link.as_ref().ok_or_else(|| {
                    DatasourceError::message(format!(
                        "Deep write requires a CRUD repository for \"{}\"",
                        enrichment.destination_property
                    ))
                })?;
                for parent in parents.iter_mut() {
                    if self.skip(parent, &enrichment.destination_property, mode) {
                        continue;
                    }
                    match mode {
                        WriteMode::Create => {
                            self.create
                                .create_linked(writer.clone(), enrichment, parent)
                                .await?
                        }
                        WriteMode::Update => {
                            self.update
                                .update_linked(writer.clone(), enrichment, parent)
                                .await?
                        }
                        WriteMode::Delete => {
                            self.delete
                                .delete_linked(writer.clone(), enrichment, parent)
                                .await?
                        }
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
            let writer = entry.persist.as_ref().ok_or_else(|| {
                DatasourceError::message(format!(
                    "Deep write requires a CRUD repository for \"{}\"",
                    enrichment.destination_property
                ))
            })?;
            for parent in parents.iter_mut() {
                if self.skip(parent, &enrichment.destination_property, mode) {
                    continue;
                }
                match mode {
                    WriteMode::Create => {
                        self.create
                            .create_owned(writer.clone(), enrichment, parent)
                            .await?
                    }
                    WriteMode::Update => {
                        self.update
                            .update_owned(writer.clone(), enrichment, parent)
                            .await?
                    }
                    WriteMode::Delete => {
                        self.delete
                            .delete_owned(writer.clone(), enrichment, parent, self)
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
}

#[async_trait]
impl crate::delete_nested::DeleteNested for DefaultWriteEnrichmentStrategy {
    async fn delete_nested(&self, _rows: &mut [Row]) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn related_ids(parent: &Row, enrichment: &Enrichment) -> Result<Vec<Value>> {
    let mut ids = Vec::new();
    let mut seen = HashSet::new();
    for related in decode_rows(parent.get(&enrichment.destination_property)) {
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

pub(crate) fn required(parent: &Row, name: &str, into: &str) -> Result<Value> {
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

fn assigned(value: &Value) -> bool {
    match value {
        Value::Null | Value::Int(0) => false,
        Value::Text(text) if text.is_empty() => false,
        _ => true,
    }
}

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

pub(crate) fn decode_rows(value: Option<&Value>) -> Vec<Row> {
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
