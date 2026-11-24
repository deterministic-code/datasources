use crate::entity::{assign_one, assign_related, Entity};
use crate::enrichment::Enrichment;
use crate::repository::IReadonlyRepository;
use crate::result::Result;
use crate::row::Row;
use crate::schema::Schema;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

const DEFAULT_FILL_BATCH_SIZE: usize = 500;

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
                self.apply_dictionary(via, enrichment, parents, sources)
                    .await
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
        let loaded = load_by_keys(
            via,
            &enrichment.related_key,
            &distinct_keys(parents, sources, enrichment),
            self.fill_batch_size(),
        )
        .await?;
        let mut index = HashMap::new();
        for related in loaded {
            if let Some(cache_key) = row_match_key(&related, enrichment) {
                index.entry(cache_key).or_insert(related);
            }
        }
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(cache_key) = match_key(parent, enrichment, source_at(sources, index_i)) else {
                continue;
            };
            let Some(related) = index.get(&cache_key).cloned() else {
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
        let index = group(
            load_by_keys(
                via,
                &enrichment.related_key,
                &distinct_keys(parents, sources, enrichment),
                self.fill_batch_size(),
            )
            .await?,
            enrichment,
        );
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(cache_key) = match_key(parent, enrichment, source_at(sources, index_i)) else {
                continue;
            };
            let related = index.get(&cache_key).cloned().unwrap_or_default();
            assign_related(parent, &enrichment.destination_property, related)?;
        }
        Ok(())
    }

    async fn apply_dictionary(
        &self,
        via: Arc<dyn IReadonlyRepository<TRelated>>,
        enrichment: &Enrichment,
        parents: &mut Vec<T>,
        sources: Option<&[Option<Row>]>,
    ) -> Result<()> {
        let schema = via.schema().clone();
        let index = group(
            load_by_keys(
                via,
                &enrichment.related_key,
                &distinct_keys(parents, sources, enrichment),
                self.fill_batch_size(),
            )
            .await?,
            enrichment,
        );
        for (index_i, parent) in parents.iter_mut().enumerate() {
            let Some(cache_key) = match_key(parent, enrichment, source_at(sources, index_i)) else {
                continue;
            };
            let related = index.get(&cache_key).cloned().unwrap_or_default();
            let rows = shape_dictionary_rows(related, enrichment, &schema)?;
            parent.set_payload(&enrichment.destination_property, rows)?;
        }
        Ok(())
    }

    fn fill_batch_size(&self) -> usize {
        DEFAULT_FILL_BATCH_SIZE
    }

    fn linked_batch_size(&self) -> Option<usize> {
        None
    }
}

pub(crate) async fn load_by_keys<TItem: Entity>(
    via: Arc<dyn IReadonlyRepository<TItem>>,
    column: &str,
    keys: &[Value],
    batch_size: usize,
) -> Result<Vec<TItem>> {
    let mut related = Vec::new();
    if keys.is_empty() {
        return Ok(related);
    }
    for chunk in keys.chunks(batch_size.max(1)) {
        let loaded = via
            .read_where(
                &Where::new(Operator::in_list(column.to_string(), chunk.to_vec())),
                None,
            )
            .try_collect::<Vec<_>>()
            .await?;
        related.extend(loaded);
    }
    Ok(related)
}

pub(crate) fn distinct_keys<T: Entity>(
    parents: &[T],
    sources: Option<&[Option<Row>]>,
    enrichment: &Enrichment,
) -> Vec<Value> {
    let mut keys = Vec::new();
    let mut seen = HashSet::new();
    for (index, parent) in parents.iter().enumerate() {
        let Some(key) = source_key_of(parent, &enrichment.source_property, source_at(sources, index))
        else {
            continue;
        };
        if seen.insert(key_of(&key)) {
            keys.push(key);
        }
    }
    keys
}

pub(crate) fn group<TRelated: Entity>(
    items: Vec<TRelated>,
    enrichment: &Enrichment,
) -> HashMap<String, Vec<TRelated>> {
    let mut index: HashMap<String, Vec<TRelated>> = HashMap::new();
    for related in items {
        if let Some(cache_key) = row_match_key(&related, enrichment) {
            index.entry(cache_key).or_default().push(related);
        }
    }
    index
}

pub(crate) fn shape_dictionary_rows<TRelated: Entity>(
    related: Vec<TRelated>,
    enrichment: &Enrichment,
    schema: &Schema,
) -> Result<Vec<Row>> {
    let key_name = enrichment.map_key()?;
    let value_columns = enrichment.value_columns(schema)?;
    Ok(related
        .into_iter()
        .filter_map(|item| {
            let key = item.cell(key_name)?;
            let mut row = Row::new();
            row.insert("key".to_string(), key.clone());
            row.insert(key_name.to_string(), key);
            if let Some(select) = enrichment.related_property.as_deref() {
                row.insert(select.to_string(), item.cell(select)?);
            } else {
                for name in &value_columns {
                    if let Some(cell) = item.cell(name) {
                        row.insert(name.clone(), cell);
                    }
                }
            }
            Some(row)
        })
        .collect())
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
            .read_where(
                &Where::new(Operator::equals(association.parent.clone(), key)),
                None,
            )
            .try_collect::<Vec<_>>()
            .await?;
        let mut related = Vec::new();
        for row in links {
            let Some(id) = row.cell(&association.related) else {
                continue;
            };
            if let Some(entity) = via
                .read_single_where(
                    &Where::new(Operator::equals(
                        enrichment.related_key.clone(),
                        id,
                    )),
                    None,
                )
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

pub(crate) fn where_equals(pairs: impl IntoIterator<Item = (String, Value)>) -> Option<Where> {
    let mut condition: Option<Operator> = None;
    for (column, value) in pairs {
        let equals = Operator::equals(column, value);
        condition = Some(match condition {
            None => equals,
            Some(left) => Operator::and(left, equals),
        });
    }
    condition.map(Where::new)
}

pub(crate) fn match_key<T: Entity>(
    parent: &T,
    enrichment: &Enrichment,
    source: Option<&Row>,
) -> Option<String> {
    let mut parts = Vec::new();
    for (source_name, _) in enrichment.pairs() {
        parts.push(key_of(&source_key_of(parent, &source_name, source)?));
    }
    Some(parts.join("\u{0}"))
}

pub(crate) fn row_match_key<T: Entity>(row: &T, enrichment: &Enrichment) -> Option<String> {
    let mut parts = Vec::new();
    for (_, related) in enrichment.pairs() {
        let key = row.cell(&related).filter(|value| *value != Value::Null)?;
        parts.push(key_of(&key));
    }
    Some(parts.join("\u{0}"))
}
