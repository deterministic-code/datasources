use crate::datasource_error::DatasourceError;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::value::Value;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, OnceLock};

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

static LOOKUPS: OnceLock<Mutex<HashMap<usize, Row>>> = OnceLock::new();

fn lookups() -> &'static Mutex<HashMap<usize, Row>> {
    LOOKUPS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn item_key<T>(item: &T) -> usize {
    std::ptr::from_ref(item) as usize
}

fn missing(value: &Value) -> bool {
    matches!(value, Value::Null | Value::Int(0))
        || matches!(value, Value::Text(text) if text.is_empty())
}

pub(crate) fn try_get<T: Entity>(item: &T, name: &str) -> Option<Value> {
    item.cell(name)
}

pub(crate) fn key(value: Option<Value>) -> Option<Value> {
    match value {
        None => None,
        Some(Value::Null) => Some(Value::Null),
        Some(other) => Some(other),
    }
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
    if T::is_dictionary() || item.has(name) {
        return item.set_cell(name, value);
    }
    lookups()
        .lock()
        .expect("lookups")
        .entry(item_key(item))
        .or_default()
        .insert(name.to_string(), value);
    Ok(())
}

pub(crate) fn merge_lookups<T: Entity>(item: &T, record: &mut Row) {
    let Some(extras) = lookups().lock().expect("lookups").get(&item_key(item)).cloned() else {
        return;
    };
    for (key, value) in extras {
        record.insert(key, value);
    }
}

#[allow(dead_code)]
pub(crate) fn lookup_keys<T: Entity>(item: &T) -> HashSet<String> {
    lookups()
        .lock()
        .expect("lookups")
        .get(&item_key(item))
        .map(|row| row.keys().cloned().collect())
        .unwrap_or_default()
}

pub(crate) async fn resolve_lookups<T: Entity>(
    entries: &[StoredEnrichment<T>],
    items: &mut [T],
) -> Result<()> {
    for item in items.iter_mut() {
        for entry in entries {
            let enrichment = &entry.enrichment;
            if enrichment.r#type != crate::EnrichmentPropertyType::Object
                || enrichment.association.is_some()
            {
                continue;
            }
            let Some(related) = enrichment.related_property.as_deref() else {
                continue;
            };
            let Some(name) = try_get(item, &enrichment.destination_property).filter(|value| !missing(value))
            else {
                continue;
            };
            if key(try_get(item, &enrichment.source_property)).is_some_and(|value| !missing(&value))
            {
                continue;
            }
            let found = entry
                .lookup(related, name.clone(), &enrichment.related_key)
                .await?;
            let id = found
                .filter(|value| !missing(value))
                .ok_or_else(|| {
                    DatasourceError::message(format!(
                        "Unknown {} \"{}\"",
                        enrichment.destination_property,
                        name.wire_text()
                    ))
                })?;
            set(item, &enrichment.source_property, id)?;
        }
    }
    Ok(())
}
