use futures_util::TryStreamExt;
use crate::delete_nested::DeleteNested;
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::{Operator, Where};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DeleteEnrichmentStrategy: Send + Sync {
    async fn delete_children(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &Row,
        delete_nested: &dyn DeleteNested,
    ) -> Result<()>;

    async fn delete_linked(
        &self,
        writer: Arc<dyn ICRUDRepository>,
        enrichment: &Enrichment,
        parent: &Row,
    ) -> Result<()>;
}

pub(crate) async fn load(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
) -> Result<Vec<Row>> {
    let Some(key) = parent.get(&enrichment.source_property).cloned() else {
        return Ok(Vec::new());
    };
    writer
        .read(
            Some(&Where::new(Operator::equals(enrichment.related_key.clone(), key))),
            None, None
        )
        .try_collect::<Vec<_>>().await
}

pub(crate) fn children_where(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
) -> Option<Where> {
    if !writer.id_names().is_empty() {
        return None;
    }
    parent
        .get(&enrichment.source_property)
        .cloned()
        .map(|key| Where::new(Operator::equals(enrichment.related_key.clone(), key)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crud_repository::CRUDRepository;
    use crate::default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
    use crate::repository::IReadonlyRepository;
    use crate::value::Value;
    use std::sync::Arc;

    struct SkipDelete;

    #[async_trait]
    impl DeleteEnrichmentStrategy for SkipDelete {
        async fn delete_children(
            &self,
            _writer: Arc<dyn ICRUDRepository>,
            _enrichment: &Enrichment,
            _parent: &Row,
            _delete_nested: &dyn DeleteNested,
        ) -> Result<()> {
            Ok(())
        }

        async fn delete_linked(
            &self,
            _writer: Arc<dyn ICRUDRepository>,
            _enrichment: &Enrichment,
            _parent: &Row,
        ) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn delete_strategy_can_skip_children() {
        let write = Arc::new(DefaultWriteEnrichmentStrategy::new(
            Some(Arc::new(SkipDelete)),
            None,
            None,
            false,
        ));
        let children = Arc::new(CRUDRepository::<Row>::memory(vec![]));
        let mut parents = CRUDRepository::<Row>::memory_with(vec![], Some(write));
        let parent = Row::new();
        let created = parents.create(parent).await.unwrap();
        let mut child = Row::new();
        child.insert("parent_id".into(), created.get("id").unwrap().clone());
        child.insert("label".into(), Value::from("a"));
        children.create(child).await.unwrap();
        parents.add_enriched(
            Enrichment::new("children", "id", "parent_id", None, crate::EnrichmentPropertyType::List),
            children.clone(),
            None,
        );
        parents.delete(created, None).await.unwrap();
        let left = children.read(None, None, None).try_collect::<Vec<_>>().await.unwrap();
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].get("label"), Some(&Value::from("a")));
        assert!(parents.read(None, None, None).try_collect::<Vec<_>>().await.unwrap().is_empty());
    }
}
