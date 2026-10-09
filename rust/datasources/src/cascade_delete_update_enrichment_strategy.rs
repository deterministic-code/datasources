use crate::default_create_enrichment_strategy::{create_children, create_links};
use crate::default_write_enrichment_strategy::required;
use crate::delete_enrichment_strategy::{load, children_where};
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CascadeDeleteUpdateEnrichmentStrategy;

#[async_trait]
impl UpdateEnrichmentStrategy for CascadeDeleteUpdateEnrichmentStrategy {
    async fn update_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        let existing = load(writer.as_ref(), enrichment, parent).await?;
        for row in &existing {
            match children_where(writer.as_ref(), enrichment, parent) {
                Some(clause) => writer.delete(row, Some(&clause)).await?,
                None => writer.delete(row, None).await?,
            };
        }
        create_children(writer.as_ref(), enrichment, parent).await
    }

    async fn update_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &mut Row,
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
            .read(
                Some(&Where::new(Operator::equals(
                    association.parent.clone(),
                    parent_key.clone(),
                ))),
                None, None
        )
            .await?;
        for row in &existing {
            let Some(related) = row.get(&association.related).cloned() else {
                continue;
            };
            if related == Value::Null {
                continue;
            }
            let clause = Where::new(Operator::and(
                Operator::equals(association.parent.clone(), parent_key.clone()),
                Operator::equals(association.related.clone(), related),
            ));
            writer.delete(row, Some(&clause)).await?;
        }
        create_links(writer.as_ref(), enrichment, parent).await
    }
}
