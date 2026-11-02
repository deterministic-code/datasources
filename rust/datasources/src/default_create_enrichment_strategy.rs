use crate::create_enrichment_strategy::{set, CreateEnrichmentStrategy};
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::{related_ids, required};
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct DefaultCreateEnrichmentStrategy;

#[async_trait]
impl<T: Entity> CreateEnrichmentStrategy<T> for DefaultCreateEnrichmentStrategy {
    async fn create_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        create_children(writer.as_ref(), enrichment, parent).await
    }

    async fn create_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        create_links(writer.as_ref(), enrichment, parent).await
    }
}

pub(crate) async fn create_children<T: Entity, TChild: Entity>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &mut T,
) -> Result<()> {
    let mut related = crate::create_enrichment_strategy::children::<T, TChild>(enrichment, parent)?;
    if enrichment.r#type == crate::EnrichmentPropertyType::Dictionary {
        related.sort_by(|left, right| dict_key(left).cmp(&dict_key(right)));
    }
    let key = parent.cell(&enrichment.source_property);
    for child in &mut related {
        if let Some(key) = &key {
            set(child, &enrichment.related_key, key.clone())?;
        }
        writer.create(child.clone()).await?;
    }
    Ok(())
}

pub(crate) async fn create_links<T: Entity, TLink: Entity>(
    writer: &dyn ICRUDRepository<TLink>,
    enrichment: &Enrichment,
    parent: &T,
) -> Result<()> {
    let association = enrichment
        .association
        .as_ref()
        .ok_or_else(|| DatasourceError::message("Missing association"))?;
    let parent_key = required(
        parent,
        &enrichment.source_property,
        &enrichment.destination_property,
    )?;
    for id in related_ids(parent, enrichment)? {
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
