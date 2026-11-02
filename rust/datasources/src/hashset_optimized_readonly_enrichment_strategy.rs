use crate::default_write_enrichment_strategy::{encode_row, encode_rows};
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::readonly_enrichment_strategy::{
    key_of, source_at, source_key_t, set_dest, ReadonlyEnrichmentStrategy,
};
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

pub struct HashsetOptimizedReadonlyEnrichmentStrategy;

#[async_trait]
impl<T: FromSchema, TRelated: FromSchema> ReadonlyEnrichmentStrategy<T, TRelated>
    for HashsetOptimizedReadonlyEnrichmentStrategy
{
    async fn apply(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        match enrichment.r#type {
            crate::EnrichmentPropertyType::Object => {
                apply_object(via, enrichment, parents, sources).await
            }
            crate::EnrichmentPropertyType::List | crate::EnrichmentPropertyType::Dictionary => {
                apply_list(via, enrichment, parents, sources).await
            }
            crate::EnrichmentPropertyType::Custom => Err(crate::DatasourceError::message(format!(
                "Unsupported enrichment type \"{:?}\"",
                enrichment.r#type
            ))),
        }
    }
}

async fn apply_object<T: FromSchema, TRelated: FromSchema>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    let keys = keys(parents, sources, &enrichment.source_property)?;
    if keys.is_empty() {
        return Ok(());
    }
    let loaded = via
        .read(&Where::new(Operator::in_list(
            enrichment.related_key.clone(),
            keys.values().cloned().collect(),
        )))
        .try_collect::<Vec<_>>()
        .await?;
    let mut index = HashMap::new();
    for related in loaded {
        let row = related.to_schema(&Schema::default())?;
        if let Some(by) = row.get(&enrichment.related_key) {
            index.insert(key_of(by), (related, row));
        }
    }
    for (index_i, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key_t(parent, &enrichment.source_property, source_at(sources, index_i))?
        else {
            continue;
        };
        let Some((_, row)) = index.get(&key_of(&key)) else {
            continue;
        };
        let value = enrichment
            .related_property
            .as_ref()
            .and_then(|select| row.get(select).cloned())
            .unwrap_or_else(|| encode_row(row));
        set_dest(parent, &enrichment.destination_property, value)?;
    }
    Ok(())
}

async fn apply_list<T: FromSchema, TRelated: FromSchema>(
    via: Arc<dyn IReadonlyRepository<TRelated>>,
    enrichment: &Enrichment,
    parents: &mut Vec<T>,
    sources: Option<&[Option<Row>]>,
) -> Result<()> {
    let keys = keys(parents, sources, &enrichment.source_property)?;
    if keys.is_empty() {
        return Ok(());
    }
    let loaded = via
        .read(&Where::new(Operator::in_list(
            enrichment.related_key.clone(),
            keys.values().cloned().collect(),
        )))
        .try_collect::<Vec<_>>()
        .await?;
    let mut index: HashMap<String, Vec<Row>> = HashMap::new();
    for related in loaded {
        let row = related.to_schema(&Schema::default())?;
        if let Some(by) = row.get(&enrichment.related_key) {
            index.entry(key_of(by)).or_default().push(row);
        }
    }
    for (index_i, parent) in parents.iter_mut().enumerate() {
        let Some(key) = source_key_t(parent, &enrichment.source_property, source_at(sources, index_i))?
        else {
            continue;
        };
        let related = index.get(&key_of(&key)).cloned().unwrap_or_default();
        set_dest(
            parent,
            &enrichment.destination_property,
            encode_rows(&related),
        )?;
    }
    Ok(())
}

fn keys<T: FromSchema>(
    parents: &[T],
    sources: Option<&[Option<Row>]>,
    from: &str,
) -> Result<HashMap<String, Value>> {
    let mut keys = HashMap::new();
    for (index, parent) in parents.iter().enumerate() {
        if let Some(key) = source_key_t(parent, from, source_at(sources, index))? {
            keys.entry(key_of(&key)).or_insert(key);
        }
    }
    Ok(keys)
}
