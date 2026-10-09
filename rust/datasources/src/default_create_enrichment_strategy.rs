use crate::create_enrichment_strategy::CreateEnrichmentStrategy;
use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::{
    decode_rows, encode_rows, related_ids, required,
};
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use async_trait::async_trait;
use std::sync::Arc;

pub struct DefaultCreateEnrichmentStrategy;

#[async_trait]
impl CreateEnrichmentStrategy for DefaultCreateEnrichmentStrategy {
    async fn create_owned(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        create_children(writer.as_ref(), enrichment, parent).await
    }

    async fn create_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        create_links(writer.as_ref(), enrichment, parent).await
    }
}

pub(crate) async fn create_children(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &mut Row,
) -> Result<()> {
    let mut related = decode_rows(parent.get(&enrichment.destination_property));
    if enrichment.r#type == crate::EnrichmentPropertyType::Dictionary {
        related.sort_by(|left, right| dict_key(left).cmp(&dict_key(right)));
    }
    let key = parent.get(&enrichment.source_property).cloned();
    for row in &mut related {
        if let Some(key) = &key {
            row.insert(enrichment.related_key.clone(), key.clone());
        }
        writer.create(row).await?;
    }
    parent.insert(
        enrichment.destination_property.clone(),
        encode_rows(&related),
    );
    Ok(())
}

pub(crate) async fn create_links(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
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
        let mut row = Row::new();
        row.insert(association.parent.clone(), parent_key.clone());
        row.insert(association.related.clone(), id);
        writer.create(&mut row).await?;
    }
    Ok(())
}

fn dict_key(row: &Row) -> String {
    row.get("Key")
        .or_else(|| row.get("key"))
        .map(Value::wire_text)
        .unwrap_or_default()
}
