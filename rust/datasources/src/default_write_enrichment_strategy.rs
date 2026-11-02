use crate::cascade_delete_enrichment_strategy::CascadeDeleteEnrichmentStrategy;
use crate::create_enrichment_strategy::CreateEnrichmentStrategy;
use crate::datasource_error::DatasourceError;
use crate::default_create_enrichment_strategy::DefaultCreateEnrichmentStrategy;
use crate::default_update_enrichment_strategy::DefaultUpdateEnrichmentStrategy;
use crate::delete_enrichment_strategy::DeleteEnrichmentStrategy;
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::repository_tree::RepositoryTree;
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::stored_enrichment::WriteEnrichment;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use crate::value::Value;
use crate::write_enrichment_strategy::WriteEnrichmentStrategy;
use crate::write_mode::WriteMode;
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

pub struct DefaultWriteEnrichmentStrategy<T: FromSchema = Row> {
    create: Arc<dyn CreateEnrichmentStrategy<T>>,
    delete: Arc<dyn DeleteEnrichmentStrategy<T>>,
    update: Arc<dyn UpdateEnrichmentStrategy<T>>,
    skip_unloaded: bool,
    use_transaction: bool,
}

impl<T: FromSchema> DefaultWriteEnrichmentStrategy<T> {
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

impl<T: FromSchema> Default for DefaultWriteEnrichmentStrategy<T> {
    fn default() -> Self {
        Self::new(None, None, None, false)
    }
}

#[async_trait]
impl<T: FromSchema> WriteEnrichmentStrategy<T> for DefaultWriteEnrichmentStrategy<T> {
    fn use_transaction(&self) -> bool {
        self.use_transaction
    }

    async fn resolve_lookups(&self, tree: &RepositoryTree<T>, items: &mut [T]) -> Result<()> {
        let entries = tree.parent.enrichments.lock().expect("enrichments").clone();
        for item in items.iter_mut() {
            for entry in &entries {
                let enrichment = &entry.enrichment;
                if enrichment.r#type != crate::EnrichmentPropertyType::Object
                    || enrichment.association.is_some()
                    || enrichment.related_property.is_none()
                {
                    continue;
                }
                let row = field_row(item)?;
                let Some(name) = row.get(&enrichment.destination_property).filter(|value| !missing(value)) else {
                    continue;
                };
                if row
                    .get(&enrichment.source_property)
                    .is_some_and(|value| !missing(value))
                {
                    continue;
                }
                let _ = name;
                if let Some(lookup) = &entry.lookup {
                    lookup(item).await?;
                }
            }
        }
        Ok(())
    }

    async fn persist(
        &self,
        tree: &RepositoryTree<T>,
        items: &mut [T],
        mode: WriteMode,
    ) -> Result<()> {
        let entries = tree.parent.enrichments.lock().expect("enrichments").clone();
        for entry in &entries {
            self.persist_entry(entry, items, mode).await?;
        }
        Ok(())
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
        for entry in &entries {
            let enrichment = &entry.enrichment;
            if enrichment.association.is_some()
                || (enrichment.r#type == crate::EnrichmentPropertyType::List
                    && enrichment.related_property.is_some())
                || (enrichment.r#type != crate::EnrichmentPropertyType::List
                    && enrichment.r#type != crate::EnrichmentPropertyType::Dictionary)
            {
                self.persist_entry(entry, items, mode).await?;
                continue;
            }
            let Some(writes) = &entry.persist else {
                self.persist_entry(entry, items, mode).await?;
                continue;
            };
            let mut children = Vec::new();
            for parent in items.iter() {
                if self.skip(parent, &enrichment.destination_property, mode)? {
                    continue;
                }
                children.extend(collect_children(parent, enrichment)?);
            }
            if children.is_empty() {
                continue;
            }
            if !writes.write_collected(&mut children, mode).await? {
                self.persist_entry(entry, items, mode).await?;
            }
        }
        Ok(())
    }
}

pub(crate) fn collect_children<T: FromSchema>(parent: &T, enrichment: &Enrichment) -> Result<Vec<Row>> {
    let row = field_row(parent)?;
    let mut related = decode_rows(row.get(&enrichment.destination_property));
    if related.is_empty() {
        return Ok(related);
    }
    let key = required_field(parent, &enrichment.source_property, &enrichment.destination_property)?;
    for child in &mut related {
        child.insert(enrichment.related_key.clone(), key.clone());
    }
    Ok(related)
}

impl<T: FromSchema> DefaultWriteEnrichmentStrategy<T> {
    async fn persist_entry(
        &self,
        entry: &WriteEnrichment<T>,
        parents: &mut [T],
        mode: WriteMode,
    ) -> Result<()> {
        let enrichment = &entry.enrichment;
        if matches!(enrichment.r#type, crate::EnrichmentPropertyType::Custom) {
            return Err(DatasourceError::message(format!(
                "Unsupported enrichment type \"{:?}\"",
                enrichment.r#type
            )));
        }
        if enrichment.association.is_some() {
            let writes = entry.link.as_ref().ok_or_else(|| {
                DatasourceError::message(format!(
                    "Deep write requires a CRUD repository for \"{}\"",
                    enrichment.destination_property
                ))
            })?;
            for parent in parents.iter_mut() {
                if self.skip(parent, &enrichment.destination_property, mode)? {
                    continue;
                }
                match mode {
                    WriteMode::Create => self.create.create_linked(enrichment, parent, writes.as_ref()).await?,
                    WriteMode::Update => self.update.update_linked(enrichment, parent, writes.as_ref()).await?,
                    WriteMode::Delete => self.delete.delete_linked(enrichment, parent, writes.as_ref()).await?,
                }
            }
            return Ok(());
        }
        if enrichment.r#type == crate::EnrichmentPropertyType::List && enrichment.related_property.is_some() {
            return Ok(());
        }
        if enrichment.r#type != crate::EnrichmentPropertyType::List
            && enrichment.r#type != crate::EnrichmentPropertyType::Dictionary
        {
            return Ok(());
        }
        let writes = entry.persist.as_ref().ok_or_else(|| {
            DatasourceError::message(format!(
                "Deep write requires a CRUD repository for \"{}\"",
                enrichment.destination_property
            ))
        })?;
        for parent in parents.iter_mut() {
            if self.skip(parent, &enrichment.destination_property, mode)? {
                continue;
            }
            match mode {
                WriteMode::Create => self.create.create_children(enrichment, parent, writes.as_ref()).await?,
                WriteMode::Update => self.update.update_children(enrichment, parent, writes.as_ref()).await?,
                WriteMode::Delete => self.delete.delete_children(enrichment, parent, writes.as_ref()).await?,
            }
        }
        Ok(())
    }

    fn skip(&self, parent: &T, into: &str, mode: WriteMode) -> Result<bool> {
        if !self.skip_unloaded || mode == WriteMode::Delete {
            return Ok(false);
        }
        Ok(!field_row(parent)?.contains_key(into))
    }
}

#[async_trait]
impl<T: FromSchema> crate::delete_nested::DeleteNested for DefaultWriteEnrichmentStrategy<T> {
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

pub(crate) fn field_row<T: FromSchema>(item: &T) -> Result<Row> {
    item.to_schema(&Schema::default())
}

pub(crate) fn required_field<T: FromSchema>(parent: &T, name: &str, into: &str) -> Result<Value> {
    field_row(parent)?
        .get(name)
        .filter(|value| !missing(value))
        .cloned()
        .ok_or_else(|| DatasourceError::message(format!("{into} requires \"{name}\"")))
}

pub(crate) fn missing(value: &Value) -> bool {
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
