use crate::default_write_enrichment_strategy::{encode_row, encode_rows};
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait ReadonlyEnrichmentStrategy<T: FromSchema, TRelated: FromSchema>: Send + Sync {
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        match enrichment.r#type {
            crate::EnrichmentPropertyType::List | crate::EnrichmentPropertyType::Dictionary => {
                apply_list(via, enrichment, parents, sources).await
            }
            crate::EnrichmentPropertyType::Object => {
                apply_object(via, enrichment, parents, sources).await
            }
            crate::EnrichmentPropertyType::Custom => Err(crate::DatasourceError::message(format!(
                "Unsupported enrichment type \"{:?}\"",
                enrichment.r#type
            ))),
        }
    }
}

pub(crate) async fn apply_object<T: FromSchema, TRelated: FromSchema>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    let mut cache: HashMap<String, Option<TRelated>> = HashMap::new();
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key_t(parent, &enrichment.source_property, source_at(sources, index))?
        else {
            continue;
        };
        let cache_key = key_of(&key);
        if !cache.contains_key(&cache_key) {
            let related = via
                .read_single(&Where::new(Operator::equals(
                    enrichment.related_key.clone(),
                    key.clone(),
                )))
                .await?;
            cache.insert(cache_key.clone(), related);
        }
        let Some(related) = cache.get(&cache_key).cloned().flatten() else {
            continue;
        };
        set_dest(
            parent,
            &enrichment.destination_property,
            related_cell(&related, enrichment)?,
        )?;
    }
    Ok(())
}

pub(crate) async fn apply_list<T: FromSchema, TRelated: FromSchema>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key_t(parent, &enrichment.source_property, source_at(sources, index))?
        else {
            continue;
        };
        let related = via
            .read(&Where::new(Operator::equals(
                enrichment.related_key.clone(),
                key,
            )))
            .try_collect::<Vec<_>>()
            .await?;
        set_dest(
            parent,
            &enrichment.destination_property,
            encode_related(&related)?,
        )?;
    }
    Ok(())
}

pub(crate) async fn apply_linked<T: FromSchema, TRelated: FromSchema, TLink: FromSchema>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    link: Arc<dyn IReadonlyRepository<TLink>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    let association = enrichment.association.as_ref().expect("association");
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key_t(parent, &enrichment.source_property, source_at(sources, index))?
        else {
            continue;
        };
        let links = link
            .read(&Where::new(Operator::equals(association.parent.clone(), key)))
            .try_collect::<Vec<_>>()
            .await?;
        let mut related_rows = Vec::new();
        for row in links {
            let link_row = row.to_schema(&Schema::default())?;
            let Some(id) = link_row.get(&association.related).cloned() else {
                continue;
            };
            if let Some(entity) = via
                .read_single(&Where::new(Operator::equals(
                    enrichment.related_key.clone(),
                    id,
                )))
                .await?
            {
                related_rows.push(entity.to_schema(&Schema::default())?);
            }
        }
        set_dest(
            parent,
            &enrichment.destination_property,
            encode_rows(&related_rows),
        )?;
    }
    Ok(())
}

fn related_cell<TRelated: FromSchema>(item: &TRelated, enrichment: &Enrichment) -> Result<Value> {
    let row = item.to_schema(&Schema::default())?;
    Ok(selected(enrichment, &row))
}

fn encode_related<TRelated: FromSchema>(related: &[TRelated]) -> Result<Value> {
    let rows = related
        .iter()
        .map(|item| item.to_schema(&Schema::default()))
        .collect::<Result<Vec<_>>>()?;
    Ok(encode_rows(&rows))
}

fn selected(enrichment: &Enrichment, related: &Row) -> Value {
    enrichment
        .related_property
        .as_ref()
        .and_then(|select| related.get(select).cloned())
        .unwrap_or_else(|| encode_row(related))
}

pub(crate) fn source_key_t<T: FromSchema>(
    parent: &T,
    name: &str,
    source: Option<&Row>,
) -> Result<Option<Value>> {
    if let Some(source) = source {
        if let Some(cell) = source.get(name) {
            if *cell != Value::Null {
                return Ok(Some(cell.clone()));
            }
        }
    }
    let row = parent.to_schema(&Schema::default())?;
    Ok(source_key(&row, name, None))
}

pub(crate) fn set_dest<T: FromSchema>(parent: &mut T, name: &str, value: Value) -> Result<()> {
    let mut row = parent.to_schema(&Schema::default())?;
    row.insert(name.to_string(), value);
    *parent = crate::repository::map_row(row, &Schema::default())?;
    Ok(())
}

pub(crate) fn source_key(parent: &Row, name: &str, source: Option<&Row>) -> Option<Value> {
    if let Some(source) = source {
        if let Some(cell) = source.get(name) {
            if *cell != Value::Null {
                return Some(cell.clone());
            }
        }
    }
    parent
        .get(name)
        .filter(|value| **value != Value::Null)
        .cloned()
}

pub(crate) fn source_at<'a>(sources: Option<&'a [Option<Row>]>, index: usize) -> Option<&'a Row> {
    sources
        .and_then(|rows| rows.get(index))
        .and_then(|row| row.as_ref())
}

pub(crate) fn key_of(key: &Value) -> String {
    key.wire_text()
}
