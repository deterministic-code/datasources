use crate::datasource_error::DatasourceError;
use crate::default_write_enrichment_strategy::{create_children, create_links, required};
use crate::delete_enrichment_strategy::load;
use crate::enrichment::Enrichment;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::update_enrichment_strategy::UpdateEnrichmentStrategy;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;

pub struct CascadeDeleteUpdateEnrichmentStrategy;

#[async_trait]
impl UpdateEnrichmentStrategy for CascadeDeleteUpdateEnrichmentStrategy {
    async fn update_owned(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        let writer = entry.persist.as_ref().ok_or_else(|| {
            DatasourceError::message(format!(
                "Deep write requires a CRUD repository for \"{}\"",
                enrichment.into
            ))
        })?;
        let existing = load(writer.as_ref(), enrichment, parent).await?;
        for row in &existing {
            writer.delete(row, None).await?;
        }
        create_children(writer.as_ref(), enrichment, parent).await
    }

    async fn update_linked(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &mut Row,
    ) -> Result<()> {
        let association = enrichment
            .association
            .as_ref()
            .ok_or_else(|| DatasourceError::message("Missing association"))?;
        let writer = entry.link.as_ref().ok_or_else(|| {
            DatasourceError::message(format!(
                "Deep write requires a CRUD repository for \"{}\"",
                enrichment.into
            ))
        })?;
        let parent_key = required(parent, &enrichment.from, &enrichment.into)?;
        let existing = writer
            .read(
                Some(&Where::new(Operator::equals(
                    association.parent.clone(),
                    parent_key.clone(),
                ))),
                None,
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
