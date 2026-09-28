use crate::default_write_enrichment_strategy::{encode_row, encode_rows};
use crate::enrichment::Enrichment;
use crate::repository::{ICRUDRepository, IReadonlyRepository};
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
        if enrichment.many {
            self.apply_many(via, enrichment, parents, sources).await
        } else {
            self.apply_lookup(via, enrichment, parents, sources).await
        }
    }

    async fn apply_lookup(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let mut cache: HashMap<String, Option<Row>> = HashMap::new();
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(parent, &enrichment.from, source_at(sources, index)) else {
                continue;
            };
            let cache_key = key_of(&key);
            if !cache.contains_key(&cache_key) {
                let related = via
                    .read_single(
                        Some(&Where::new(Operator::equals(
                            enrichment.by.clone(),
                            key.clone(),
                        ))),
                        None,
                    )
                    .await?;
                cache.insert(cache_key.clone(), related);
            }
            let Some(related) = cache.get(&cache_key).cloned().flatten() else {
                continue;
            };
            parent.insert(enrichment.into.clone(), selected(enrichment, &related));
        }
        Ok(())
    }

    async fn apply_many(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(parent, &enrichment.from, source_at(sources, index)) else {
                continue;
            };
            let related = via
                .read(
                    Some(&Where::new(Operator::equals(enrichment.by.clone(), key))),
                    None,
                )
                .await?;
            parent.insert(enrichment.into.clone(), encode_rows(&related));
        }
        Ok(())
    }

    async fn apply_linked(
        &self,
        via: Arc<dyn IReadonlyRepository>,
        link: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parents: &mut Vec<Row>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let association = enrichment.association.as_ref().expect("association");
        for (index, parent) in parents.iter_mut().enumerate() {
            let Some(key) = source_key(parent, &enrichment.from, source_at(sources, index)) else {
                continue;
            };
            let links = link
                .read(
                    Some(&Where::new(Operator::equals(
                        association.parent.clone(),
                        key,
                    ))),
                    None,
                )
                .await?;
            let mut related_rows = Vec::new();
            for row in links {
                let Some(id) = row.get(&association.related).cloned() else {
                    continue;
                };
                if let Some(entity) = via
                    .read_single(
                        Some(&Where::new(Operator::equals(enrichment.by.clone(), id))),
                        None,
                    )
                    .await?
                {
                    related_rows.push(entity);
                }
            }
            parent.insert(enrichment.into.clone(), encode_rows(&related_rows));
        }
        Ok(())
    }
}

fn selected(enrichment: &Enrichment, related: &Row) -> Value {
    enrichment
        .select
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
