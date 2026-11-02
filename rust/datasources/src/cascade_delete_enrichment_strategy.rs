use futures_util::TryStreamExt;
use crate::delete_enrichment_strategy::{load, DeleteEnrichmentStrategy};
use crate::delete_nested::DeleteNested;
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::sync::Arc;

pub struct CascadeDeleteEnrichmentStrategy;

#[async_trait]
impl DeleteEnrichmentStrategy for CascadeDeleteEnrichmentStrategy {
    async fn delete_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &Row,
        delete_nested: &dyn DeleteNested,
    ) -> Result<()> {
        let mut existing = load(writer.as_ref(), enrichment, parent).await?;
        delete_nested.delete_nested(&mut existing).await?;
        for row in existing {
            writer.delete(row, None).await?;
        }
        Ok(())
    }

    async fn delete_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &Row,
    ) -> Result<()> {
        let association = enrichment
            .association
            .clone()
            .ok_or_else(|| crate::DatasourceError::message("Missing association"))?;
        let Some(parent_key) = parent.get(&enrichment.source_property).cloned() else {
            return Ok(());
        };
        if parent_key == Value::Null {
            return Ok(());
        }
        let existing = writer
            .read(&Where::new(Operator::equals(
                    association.parent.clone(),
                    parent_key.clone(),
                )))
            .try_collect::<Vec<_>>().await?;
        for row in existing {
            if row.get(&association.related).is_none() {
                continue;
            }
            writer.delete(row, None).await?;
        }
        Ok(())
    }
}
