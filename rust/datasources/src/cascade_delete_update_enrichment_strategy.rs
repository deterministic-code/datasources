use futures_util::TryStreamExt;
use crate::default_create_enrichment_strategy::{create_children, create_links};
use crate::default_write_enrichment_strategy::required;
use crate::delete_enrichment_strategy::{children_where, link_where, load, predicate};
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CascadeDeleteUpdateEnrichmentStrategy;

#[async_trait]
impl<T: Entity> UpdateEnrichmentStrategy<T> for CascadeDeleteUpdateEnrichmentStrategy {
    async fn update_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut T,
    ) -> Result<()> {
        let existing = load(writer.as_ref(), enrichment, parent).await?;
        let clause = children_where(writer.as_ref(), enrichment, parent);
        let by_parent = predicate(&clause);
        for row in &existing {
            writer.delete(row.clone(), by_parent).await?;
        }
        create_children(writer.as_ref(), enrichment, parent).await
    }

    async fn update_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
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
            .read_where(&Where::new(Operator::equals(
                    association.parent.clone(),
                    parent_key.clone(),
                )), None)
            .try_collect::<Vec<_>>().await?;
        for row in &existing {
            let related = row.cell(&association.related).unwrap_or(Value::Null);
            let clause = link_where(association, &parent_key, &related);
            writer.delete(row.clone(), Some(&clause)).await?;
        }
        create_links(writer.as_ref(), enrichment, parent).await
    }
}
