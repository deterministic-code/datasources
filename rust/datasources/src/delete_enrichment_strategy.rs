use crate::child_writes::ChildWrites;
use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::result::Result;
use async_trait::async_trait;

#[async_trait]
pub trait DeleteEnrichmentStrategy<T: FromSchema>: Send + Sync {
    async fn delete_children(
        &self,
        enrichment: &Enrichment,
        parent: &T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        let _ = self;
        writes.delete_children(enrichment, parent).await
    }

    async fn delete_linked(
        &self,
        enrichment: &Enrichment,
        parent: &T,
        writes: &dyn ChildWrites<T>,
    ) -> Result<()> {
        let _ = self;
        writes.delete_linked(enrichment, parent).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crud_repository::CRUDRepository;
    use crate::default_write_enrichment_strategy::DefaultWriteEnrichmentStrategy;
    use crate::repository::IReadonlyRepository;
    use crate::row::Row;
    use crate::value::Value;
    use futures_util::TryStreamExt;
    use std::sync::Arc;

    struct SkipDelete;

    #[async_trait]
    impl<T: FromSchema> DeleteEnrichmentStrategy<T> for SkipDelete {
        async fn delete_children(
            &self,
            _enrichment: &Enrichment,
            _parent: &T,
            _writes: &dyn ChildWrites<T>,
        ) -> Result<()> {
            Ok(())
        }

        async fn delete_linked(
            &self,
            _enrichment: &Enrichment,
            _parent: &T,
            _writes: &dyn ChildWrites<T>,
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
        let parents = CRUDRepository::<Row>::memory_with(vec![], Some(write));
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
        parents.delete(created).await.unwrap();
        let left = children.read(&()).try_collect::<Vec<_>>().await.unwrap();
        assert_eq!(left.len(), 1);
        assert_eq!(left[0].get("label"), Some(&Value::from("a")));
        assert!(parents.read(&()).try_collect::<Vec<_>>().await.unwrap().is_empty());
    }
}
