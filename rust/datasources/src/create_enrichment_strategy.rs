use crate::datasource_error::DatasourceError;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::stored_enrichment::StoredEnrichment;
use crate::value::Value;
use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

#[async_trait]
pub trait CreateEnrichmentStrategy<T: Entity = Row>: Send + Sync {
    async fn create_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()>;

    async fn create_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()>;
}

pub(crate) fn missing(value: &Value) -> bool {
    matches!(value, Value::Null | Value::Int(0))
        || matches!(value, Value::Text(text) if text.is_empty())
}

pub(crate) fn children<T: Entity, TChild: Entity>(
    enrichment: &Enrichment,
    parent: &T,
    schema: &Schema,
) -> Result<Vec<TChild>> {
    if enrichment.r#type == crate::EnrichmentPropertyType::Dictionary {
        dictionary_children(enrichment, parent, schema)
    } else {
        Ok(parent.related(&enrichment.destination_property))
    }
}

fn dictionary_children<T: Entity, TChild: Entity>(
    enrichment: &Enrichment,
    parent: &T,
    schema: &Schema,
) -> Result<Vec<TChild>> {
    let key_name = enrichment.map_key()?;
    parent
        .payload(&enrichment.destination_property)
        .into_iter()
        .map(|row| {
            let mut child = TChild::default();
            let key = row
                .get(key_name)
                .cloned()
                .or_else(|| row.get("key").cloned())
                .unwrap_or(Value::Null);
            set(&mut child, key_name, key)?;
            if let Some(select) = enrichment.related_property.as_deref() {
                let value = row.get(select).cloned().unwrap_or(Value::Null);
                set(&mut child, select, value)?;
            } else {
                for name in enrichment.value_columns(schema)? {
                    if let Some(cell) = row.get(&name).cloned() {
                        set(&mut child, &name, cell)?;
                    }
                }
            }
            Ok(child)
        })
        .collect()
}

pub(crate) fn dict_key<TChild: Entity>(child: &TChild, key_name: &str) -> String {
    child
        .cell(key_name)
        .map(|value| value.wire_text())
        .unwrap_or_default()
}

pub(crate) fn set<T: Entity>(item: &mut T, name: &str, value: Value) -> Result<()> {
    item.set_cell(name, value)
}

pub(crate) fn object_sources<T: Entity>(entries: &[StoredEnrichment<T>]) -> Vec<String> {
    entries
        .iter()
        .filter(|entry| {
            entry.enrichment.r#type == crate::EnrichmentPropertyType::Object
                && entry.enrichment.association.is_none()
        })
        .map(|entry| entry.enrichment.source_property.clone())
        .collect()
}

pub(crate) fn mapped_sources<T: Entity>(
    entries: &[StoredEnrichment<T>],
    column: impl Fn(&str) -> String,
) -> Vec<(String, String)> {
    object_sources(entries)
        .into_iter()
        .map(|source| {
            let mapped = column(&source);
            (source, mapped)
        })
        .collect()
}

pub(crate) fn sources<T: Entity>(
    entries: &[StoredEnrichment<T>],
    column: impl Fn(&str) -> String,
) -> HashSet<String> {
    mapped_sources(entries, column)
        .into_iter()
        .flat_map(|(source, mapped)| [source, mapped])
        .collect()
}

pub(crate) fn destination<T: Entity>(entries: &[StoredEnrichment<T>], name: &str) -> bool {
    entries
        .iter()
        .any(|entry| entry.enrichment.destination_property == name)
}

pub(crate) async fn lookup<T: Entity>(
    entries: &[StoredEnrichment<T>],
    items: &mut [T],
    record: &mut Row,
    schema: &Schema,
    columns: &[(String, String)],
) -> Result<()> {
    for (source, mapped) in columns {
        if !record.get(source).is_some_and(|value| !missing(value)) {
            if let Some(value) = record.get(mapped).cloned().filter(|value| !missing(value)) {
                record.insert(source.clone(), value);
            }
        }
    }
    resolve_lookups(entries, items, record, schema).await?;
    for (source, mapped) in columns {
        if let Some(value) = record.get(source).cloned().filter(|value| !missing(value)) {
            record.insert(mapped.clone(), value);
            if mapped != source {
                record.remove(source);
            }
        }
    }
    Ok(())
}

pub(crate) async fn resolve_lookups<T: Entity>(
    entries: &[StoredEnrichment<T>],
    items: &mut [T],
    record: &mut Row,
    schema: &Schema,
) -> Result<()> {
    for item in items.iter_mut() {
        for entry in entries {
            let enrichment = &entry.enrichment;
            let Some(related) = enrichment.related_property.as_deref() else {
                continue;
            };
            if enrichment.association.is_some() {
                resolve_linked(entry, item, related).await?;
                continue;
            }
            if enrichment.r#type != crate::EnrichmentPropertyType::Object {
                continue;
            }
            let dest = &enrichment.destination_property;
            if record
                .get(&enrichment.source_property)
                .is_some_and(|value| !missing(value))
            {
                continue;
            }
            let Some(name) = record.get(dest).cloned().filter(|value| !missing(value)) else {
                if schema
                    .columns
                    .get(dest)
                    .is_some_and(|column| !column.nullable)
                {
                    return Err(DatasourceError::message(format!("Unknown {dest}")));
                }
                continue;
            };
            let found = entry
                .lookup(related, name.clone(), &enrichment.related_key)
                .await?;
            let id = found
                .filter(|value| !missing(value))
                .ok_or_else(|| {
                    DatasourceError::message(format!("Unknown {dest} \"{}\"", name.wire_text()))
                })?;
            set(record, &enrichment.source_property, id)?;
        }
    }
    Ok(())
}

async fn resolve_linked<T: Entity>(
    entry: &StoredEnrichment<T>,
    item: &mut T,
    related: &str,
) -> Result<()> {
    let enrichment = &entry.enrichment;
    let mut rows = item.payload(&enrichment.destination_property);
    let mut changed = false;
    for row in &mut rows {
        if row
            .get(&enrichment.related_key)
            .is_some_and(|value| !missing(value))
        {
            continue;
        }
        let Some(name) = row.get(related).cloned().filter(|value| !missing(value)) else {
            continue;
        };
        let found = entry
            .lookup(related, name.clone(), &enrichment.related_key)
            .await?;
        let id = found.filter(|value| !missing(value)).ok_or_else(|| {
            DatasourceError::message(format!(
                "Unknown {} \"{}\"",
                enrichment.destination_property,
                name.wire_text()
            ))
        })?;
        row.insert(enrichment.related_key.clone(), id);
        changed = true;
    }
    if changed {
        item.set_payload(&enrichment.destination_property, rows)?;
    }
    Ok(())
}
