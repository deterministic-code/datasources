use futures_util::TryStreamExt;
use crate::delete_enrichment_strategy::{load, children_where, DeleteEnrichmentStrategy};
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
            match children_where(writer.as_ref(), enrichment, parent) {
                Some(clause) => writer.delete(&row, Some(&clause)).await?,
                None => writer.delete(&row, None).await?,
            };
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
            .read(
                Some(&Where::new(Operator::equals(
                    association.parent.clone(),
                    parent_key.clone(),
                ))),
                None, None
        )
            .try_collect::<Vec<_>>().await?;
        for row in existing {
            let Some(related) = row.get(&association.related).cloned() else {
                continue;
            };
            let clause = Where::new(Operator::and(
                Operator::equals(association.parent.clone(), parent_key.clone()),
                Operator::equals(association.related.clone(), related),
            ));
            writer.delete(&row, Some(&clause)).await?;
        }
        Ok(())
    }
}
