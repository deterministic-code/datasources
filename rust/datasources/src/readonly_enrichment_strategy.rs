use futures_util::TryStreamExt;
use crate::default_write_enrichment_strategy::{encode_row, encode_rows};
use crate::enrichment::Enrichment;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
pub trait ReadonlyEnrichmentStrategy: Send + Sync {
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        match enrichment.r#type {
            crate::EnrichmentPropertyType::List | crate::EnrichmentPropertyType::Dictionary => {
                self.apply_list(via, enrichment, parents, sources).await
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
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let mut cache: HashMap<String, Option<Row>> = HashMap::new();
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(
                parent,
                &enrichment.source_property,
                source_at(sources, index),
            ) else {
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
            parent.insert(
                enrichment.destination_property.clone(),
                selected(enrichment, &related),
            );
        }
        Ok(())
    }

    async fn apply_list(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(
                parent,
                &enrichment.source_property,
                source_at(sources, index),
            ) else {
                continue;
            };
            let related = via
                .read(&Where::new(Operator::equals(
                        enrichment.related_key.clone(),
                        key,
                    )))
                .try_collect::<Vec<_>>().await?;
            parent.insert(
                enrichment.destination_property.clone(),
                encode_rows(&related),
            );
        }
        Ok(())
    }

    async fn apply_linked(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        link: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let association = enrichment.association.as_ref().expect("association");
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(
                parent,
                &enrichment.source_property,
                source_at(sources, index),
            ) else {
                continue;
            };
            let links = link
                .read(&Where::new(Operator::equals(
                        association.parent.clone(),
                        key,
                    )))
                .try_collect::<Vec<_>>().await?;
            let mut related_rows = Vec::new();
            for row in links {
                let Some(id) = row.get(&association.related).cloned() else {
                    continue;
                };
                if let Some(entity) = via
                    .read_single(&Where::new(Operator::equals(
                            enrichment.related_key.clone(),
                            id,
                        )))
                    .await?
                {
                    related_rows.push(entity);
                }
            }
            parent.insert(
                enrichment.destination_property.clone(),
                encode_rows(&related_rows),
            );
        }
        Ok(())
    }
}

fn selected(enrichment: &Enrichment, related: &Row) -> Value {
    enrichment
        .related_property
        .as_ref()
        .and_then(|select| related.get(select).cloned())
        .unwrap_or_else(|| encode_row(related))
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
