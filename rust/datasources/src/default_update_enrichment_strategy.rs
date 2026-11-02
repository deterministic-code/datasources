use crate::create_enrichment_strategy::set;
use crate::default_write_enrichment_strategy::{related_ids, required};
use crate::delete_enrichment_strategy::load;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::collections::HashSet;
use std::sync::Arc;

pub struct DefaultUpdateEnrichmentStrategy;

#[async_trait]
impl<T: Entity> UpdateEnrichmentStrategy<T> for DefaultUpdateEnrichmentStrategy {
    async fn update_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        update_children(writer.as_ref(), enrichment, parent).await
    }

    async fn update_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        update_linked(writer.as_ref(), enrichment, parent).await
    }
}

pub(crate) async fn update_children<T: Entity, TChild: Entity>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &mut T,
) -> Result<()> {
    let existing = load(writer, enrichment, parent).await?;
    let mut related = crate::create_enrichment_strategy::children::<T, TChild>(enrichment, parent)?;
    if enrichment.r#type == crate::EnrichmentPropertyType::Dictionary {
        related.sort_by(|left, right| dict_key(left).cmp(&dict_key(right)));
    }
    if let Some(key) = parent.cell(&enrichment.source_property) {
        for child in &mut related {
            set(child, &enrichment.related_key, key.clone())?;
        }
    }
    let ids = writer.id_names();
    let keep: HashSet<String> = related.iter().filter_map(|child| child_key(child, &ids)).collect();
    for child in existing.iter().filter(|child| {
        child_key::<TChild>(*child, &ids)
            .map(|key| !keep.contains(&key))
            .unwrap_or(true)
    }) {
        writer.delete(child.clone(), None).await?;
    }
    for child in &related {
        match child_key(child, &ids) {
            Some(key)
                if existing
                    .iter()
                    .any(|stored| child_key(stored, &ids) == Some(key.clone())) =>
            {
                writer.update(child.clone(), None).await?;
            }
            _ => {
                writer.create(child.clone()).await?;
            }
        }
    }
    Ok(())
}

pub(crate) async fn update_linked<T: Entity, TLink: Entity>(
    writer: &dyn ICRUDRepository<TLink>,
    enrichment: &Enrichment,
    parent: &mut T,
) -> Result<()> {
    let association = enrichment
        .association
        .as_ref()
        .ok_or_else(|| crate::DatasourceError::message("Missing association"))?;
    let parent_key = required(
        parent,
        &enrichment.source_property,
        &enrichment.destination_property,
    )?;
    let existing = writer
        .read(&Where::new(Operator::equals(
            association.parent.clone(),
            parent_key.clone(),
        )))
        .try_collect::<Vec<_>>()
        .await?;
    let wanted = related_ids(parent, enrichment)?;
    let keep: HashSet<String> = wanted.iter().map(Value::wire_text).collect();
    for row in &existing {
        let id = row.cell(&association.related);
        if id
            .as_ref()
            .map(Value::wire_text)
            .is_some_and(|key| keep.contains(&key))
        {
            continue;
        }
        writer.delete(row.clone(), None).await?;
    }
    for id in wanted {
        if existing
            .iter()
            .any(|row| row.cell(&association.related).map(|value| value.wire_text()) == Some(id.wire_text()))
        {
            continue;
        }
        let mut row = TLink::default();
        set(&mut row, &association.parent, parent_key.clone())?;
        set(&mut row, &association.related, id)?;
        writer.create(row).await?;
    }
    Ok(())
}

fn dict_key<TChild: Entity>(child: &TChild) -> String {
    child
        .cell("Key")
        .or_else(|| child.cell("key"))
        .map(|value| value.wire_text())
        .unwrap_or_default()
}

fn child_key<TChild: Entity>(child: &TChild, ids: &[String]) -> Option<String> {
    if ids.is_empty()
        || ids.iter().any(|name| match child.cell(name) {
            Some(value) => matches!(value, Value::Null | Value::Int(0))
                || matches!(&value, Value::Text(text) if text.is_empty()),
            None => true,
        })
    {
        return None;
    }
    Some(
        ids.iter()
            .map(|name| child.cell(name).map(|value| value.wire_text()).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\u{1f}"),
    )
}
