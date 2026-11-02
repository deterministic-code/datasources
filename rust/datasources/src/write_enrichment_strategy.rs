use crate::repository_tree::RepositoryTree;
use crate::result::Result;
use crate::row::Row;
use crate::write_mode::WriteMode;
use async_trait::async_trait;

#[async_trait]
pub trait WriteEnrichmentStrategy: Send + Sync {
    fn use_transaction(&self) -> bool {
        false
    }

    async fn persist(
        &self,
        tree: &RepositoryTree,
        items: &mut [Row],
        mode: WriteMode,
    ) -> Result<()>;

    async fn persist_bulk(
        &self,
        tree: &RepositoryTree,
        items: &mut [Row],
        mode: WriteMode,
    ) -> Result<()> {
        self.persist(tree, items, mode).await
    }

    async fn resolve_lookups(&self, _tree: &RepositoryTree, _items: &mut [Row]) -> Result<()> {
        Ok(())
    }
}
