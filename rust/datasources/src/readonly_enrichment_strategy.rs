use crate::entity::{assign_one, assign_related, Entity};
use crate::enrichment::Enrichment;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait ReadonlyEnrichmentStrategy<T: Entity, TRelated: Entity = Row>: Send + Sync {
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        match enrichment.r#type {
            crate::EnrichmentPropertyType::List => {
                self.apply_list(via, enrichment, parents, sources).await
            }
            crate::EnrichmentPropertyType::Dictionary => {
                apply_dictionary(via, enrichment, parents, sources).await
            }
            crate::EnrichmentPropertyType::Object => {
                self.apply_object(via, enrichment, parents, sources).await
            }
            crate::EnrichmentPropertyType::Custom => Err(crate::DatasourceError::message(format!(
                "Unsupported enrichment type \"{:?}\"",
                enrichment.r#type
            ))),
        }
    }

    async fn apply_object(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let mut cache: HashMap<String, Option<TRelated>> = HashMap::new();
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key_of(parent, &enrichment.source_property, source_at(sources, index))
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
            match enrichment.related_property.as_deref() {
                Some(select) => {
                    if let Some(cell) = related.cell(select) {
                        parent.set_cell(&enrichment.destination_property, cell)?;
                    }
                }
                None => assign_one(parent, &enrichment.destination_property, related)?,
            }
        }
        Ok(())
    }

    async fn apply_list(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key_of(parent, &enrichment.source_property, source_at(sources, index))
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
            assign_related(parent, &enrichment.destination_property, related)?;
        }
        Ok(())
    }
}

async fn apply_dictionary<T: Entity, TRelated: Entity>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key_of(parent, &enrichment.source_property, source_at(sources, index))
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
        match enrichment.related_property.as_deref() {
            Some(select) => {
                let key_name = crate::create_enrichment_strategy::map_key::<TRelated>(&enrichment.related_key);
                let rows = related
                    .into_iter()
                    .filter_map(|item| {
                        let key = item.cell(key_name).or_else(|| item.cell("key")).or_else(|| item.cell("Key"))?;
                        let value = item.cell(select)?;
                        let mut row = Row::new();
                        row.insert(key_name.to_string(), key);
                        row.insert(select.to_string(), value);
                        Some(row)
                    })
                    .collect();
                parent.set_payload(&enrichment.destination_property, rows)?;
            }
            None => assign_related(parent, &enrichment.destination_property, related)?,
        }
    }
    Ok(())
}

pub(crate) async fn apply_linked<T: Entity, TRelated: Entity, TLink: Entity>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    link: Arc<dyn IReadonlyRepository<TLink>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    let association = enrichment.association.as_ref().expect("association");
    for (index, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key_of(parent, &enrichment.source_property, source_at(sources, index))
        else {
            continue;
        };
        let links = link
            .read(&Where::new(Operator::equals(association.parent.clone(), key)))
            .try_collect::<Vec<_>>()
            .await?;
        let mut related = Vec::new();
        for row in links {
            let Some(id) = row.cell(&association.related) else {
                continue;
            };
            if let Some(entity) = via
                .read_single(&Where::new(Operator::equals(
                    enrichment.related_key.clone(),
                    id,
                )))
                .await?
            {
                related.push(entity);
            }
        }
        assign_related(parent, &enrichment.destination_property, related)?;
    }
    Ok(())
}

pub(crate) fn source_key_of<T: Entity>(
    parent: &T,
    name: &str,
    source: Option<&Row>,
) -> Option<Value> {
    if let Some(source) = source {
        if let Some(cell) = source.get(name) {
            if *cell != Value::Null {
                return Some(cell.clone());
            }
        }
    }
    parent.cell(name).filter(|value| *value != Value::Null)
}

pub(crate) fn source_at<'a>(sources: Option<&'a [Option<Row>]>, index: usize) -> Option<&'a Row> {
    sources
        .and_then(|rows| rows.get(index))
        .and_then(|row| row.as_ref())
}

pub(crate) fn key_of(key: &Value) -> String {
    key.wire_text()
}
