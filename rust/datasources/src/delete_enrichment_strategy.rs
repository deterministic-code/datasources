use crate::delete_nested::DeleteNested;
use crate::enrichment::Enrichment;
use crate::repository::ICRUDRepository;
use crate::result::Result;
use crate::row::Row;
use crate::stored_enrichment::StoredEnrichment;
use crate::{Operator, Where};
use async_trait::async_trait;

#[async_trait]
pub trait DeleteEnrichmentStrategy: Send + Sync {
    async fn delete_owned(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &Row,
        delete_nested: &dyn DeleteNested,
    ) -> Result<()>;

    async fn delete_linked(
        &self,
        entry: &StoredEnrichment,
        enrichment: &Enrichment,
        parent: &Row,
    ) -> Result<()>;
}

pub(crate) async fn load(
    writer: &dyn ICRUDRepository,
    enrichment: &Enrichment,
    parent: &Row,
) -> Result<Vec<Row>> {
    let Some(key) = parent.get(&enrichment.from).cloned() else {
        return Ok(Vec::new());
    };
    writer
        .read(
            Some(&Where::new(Operator::equals(enrichment.by.clone(), key))),
            None,
        )
        .await
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
        async fn delete_owned(
            &self,
            _entry: &StoredEnrichment,
            _enrichment: &Enrichment,
            _parent: &Row,
            _delete_nested: &dyn DeleteNested,
        ) -> Result<()> {
            Ok(())
        }

        async fn delete_linked(
            &self,
            _entry: &StoredEnrichment,
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
            false,
        ));
        let children = Arc::new(CRUDRepository::memory(vec![]));
        let mut parents = CRUDRepository::memory_with(vec![], Some(write));
        let mut parent = Row::new();
        let created = parents.create(&mut parent).await.unwrap();
        let mut child = Row::new();
        child.insert("parent_id".into(), created.get("id").unwrap().clone());
        child.insert("label".into(), Value::from("a"));
        children.create(&mut child).await.unwrap();
        parents.add_enriched(
            Enrichment::new("children", "id", "parent_id", None, true),
            children.clone(),
            None,
        );
        parents.delete(&created, None).await.unwrap();
        let left = children.read(None, None).await.unwrap();
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].get("label"), Some(&Value::from("a")));
        assert!(parents.read(None, None).await.unwrap().is_empty());
    }
}
