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

fn missing(value: &Value) -> bool {
    matches!(value, Value::Null | Value::Int(0))
        || matches!(value, Value::Text(text) if text.is_empty())
}

pub(crate) fn map_key<TChild: Entity>(related_key: &str) -> &'static str {
    let probe = TChild::default();
    if probe.has("Key") {
        "Key"
    } else if probe.has("key") {
        "key"
    } else if probe.has("Name") {
        "Name"
    } else if probe.has("name") {
        "name"
    } else {
        match related_key.chars().next() {
            Some(first) if related_key.contains('_') || first.is_lowercase() => "key",
            _ => "Key",
        }
    }
}

pub(crate) fn children<T: Entity, TChild: Entity>(
    enrichment: &Enrichment,
    parent: &T,
) -> Result<Vec<TChild>> {
    if enrichment.r#type == crate::EnrichmentPropertyType::Dictionary {
        dictionary_children(enrichment, parent)
    } else {
        Ok(parent.related(&enrichment.destination_property))
    }
}

fn dictionary_children<T: Entity, TChild: Entity>(
    enrichment: &Enrichment,
    parent: &T,
) -> Result<Vec<TChild>> {
    let key_name = map_key::<TChild>(&enrichment.related_key);
    parent
        .payload(&enrichment.destination_property)
        .into_iter()
        .map(|row| {
            let mut child = TChild::default();
            let key = row
                .get(key_name)
                .or_else(|| row.get("key"))
                .or_else(|| row.get("Key"))
                .cloned()
                .unwrap_or(Value::Null);
            set(&mut child, key_name, key)?;
            if let Some(select) = enrichment.related_property.as_deref() {
                let value = row
                    .get(select)
                    .or_else(|| row.get("value"))
                    .or_else(|| row.get("Value"))
                    .cloned()
                    .unwrap_or(Value::Null);
                set(&mut child, select, value)?;
            } else {
                for (name, cell) in row {
                    if name.eq_ignore_ascii_case(key_name) {
                        continue;
                    }
                    set(&mut child, &name, cell)?;
                }
            }
            Ok(child)
        })
        .collect()
}

pub(crate) fn set<T: Entity>(item: &mut T, name: &str, value: Value) -> Result<()> {
    item.set_cell(name, value)
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
            if record
                .get(&enrichment.source_property)
                .is_some_and(|value| !missing(value))
            {
                continue;
            }
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
