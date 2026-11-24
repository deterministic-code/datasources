use crate::delete_enrichment_strategy::{
    children_where, link_where, load, predicate, DeleteEnrichmentStrategy,
};
use crate::delete_nested::DeleteNested;
use crate::enrichment::Enrichment;
use crate::entity::Entity;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::value::Value;
use crate::{Operator, Where};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use std::sync::Arc;

pub struct CascadeDeleteEnrichmentStrategy;

#[async_trait]
impl<T: Entity> DeleteEnrichmentStrategy<T> for CascadeDeleteEnrichmentStrategy {
    async fn delete_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &T,
        delete_nested: &dyn DeleteNested,
    ) -> Result<()> {
        delete_children(writer.as_ref(), enrichment, parent, delete_nested).await
    }

    async fn delete_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &T,
    ) -> Result<()> {
        delete_linked(writer.as_ref(), enrichment, parent).await
    }
}

pub(crate) async fn delete_children<T: Entity, TChild: Entity>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &T,
    delete_nested: &dyn DeleteNested,
) -> Result<()> {
    let existing = load(writer, enrichment, parent).await?;
    delete_children_with(writer, enrichment, parent, existing, delete_nested).await
}

/// `delete_children` for a parent whose current children were already loaded.
pub(crate) async fn delete_children_with<T: Entity, TChild: Entity>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &T,
    existing: Vec<TChild>,
    delete_nested: &dyn DeleteNested,
) -> Result<()> {
    let mut rows: Vec<crate::row::Row> = existing
        .iter()
        .filter_map(|child| TChild::object_converter().reverse_convert(child).ok())
        .collect();
    delete_nested.delete_nested(&mut rows).await?;
    delete_existing(writer, enrichment, parent, existing).await
}

/// Deletes already-loaded children of `parent`, without cascading into their own children.
pub(crate) async fn delete_existing<T: Entity, TChild: Entity>(
    writer: &dyn ICRUDRepository<TChild>,
    enrichment: &Enrichment,
    parent: &T,
    existing: Vec<TChild>,
) -> Result<()> {
    let clause = children_where(writer, enrichment, parent);
    let by_parent = predicate(&clause);
    for child in existing {
        writer.delete(child, by_parent).await?;
    }
    Ok(())
}

pub(crate) async fn delete_linked<T: Entity, TLink: Entity>(
    writer: &dyn ICRUDRepository<TLink>,
    enrichment: &Enrichment,
    parent: &T,
) -> Result<()> {
    delete_linked_with(writer, enrichment, parent, None).await
}

/// `delete_linked` for a parent whose current link rows were already loaded (`None` loads them).
pub(crate) async fn delete_linked_with<T: Entity, TLink: Entity>(
    writer: &dyn ICRUDRepository<TLink>,
    enrichment: &Enrichment,
    parent: &T,
    loaded: Option<Vec<TLink>>,
) -> Result<()> {
    let association = enrichment
        .association
        .clone()
        .ok_or_else(|| crate::DatasourceError::message("Missing association"))?;
    let Some(parent_key) = parent.cell(&enrichment.source_property) else {
        return Ok(());
    };
    if parent_key == Value::Null {
        return Ok(());
    }
    let existing = match loaded {
        Some(rows) => rows,
        None => {
            writer
                .read_where(&Where::new(Operator::equals(
                    association.parent.clone(),
                    parent_key.clone(),
                )), None)
                .try_collect::<Vec<_>>()
                .await?
        }
    };
    for row in existing {
        let related = row.cell(&association.related).unwrap_or(Value::Null);
        let clause = link_where(&association, &parent_key, &related);
        writer.delete(row, Some(&clause)).await?;
    }
    Ok(())
}
