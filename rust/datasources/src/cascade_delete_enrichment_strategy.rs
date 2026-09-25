use crate::datasource_error::DatasourceError;
use crate::delete_enrichment_strategy::{load, DeleteEnrichmentStrategy};
use crate::delete_nested::DeleteNested;
use crate::enrichment::Enrichment;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;

pub struct CascadeDeleteEnrichmentStrategy;

#[async_trait]
impl DeleteEnrichmentStrategy for CascadeDeleteEnrichmentStrategy {
    async fn delete_owned(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &Row,
        delete_nested: &dyn DeleteNested,
    ) -> Result<()> {
        let writer = entry.persist.as_ref().ok_or_else(|| {
            DatasourceError::message(format!(
                "Deep write requires a CRUD repository for \"{}\"",
                enrichment.into
            ))
        })?;
        let mut existing = load(writer.as_ref(), enrichment, parent).await?;
        delete_nested.delete_nested(&mut existing).await?;
        for row in existing {
            writer.delete(&row, None).await?;
        }
        Ok(())
    }

    async fn delete_linked(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &Row,
    ) -> Result<()> {
        let association = enrichment
            .association
            .clone()
            .ok_or_else(|| DatasourceError::message("Missing association"))?;
        let writer = entry.link.as_ref().ok_or_else(|| {
            DatasourceError::message(format!(
                "Deep write requires a CRUD repository for \"{}\"",
                enrichment.into
            ))
        })?;
        let Some(parent_key) = parent.get(&enrichment.from).cloned() else {
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
                None,
            )
            .await?;
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
