use crate::repository_tree::RepositoryTree;
use crate::result::Result;
use crate::row::Row;
use crate::write_mode::WriteMode;
use async_trait::async_trait;

#[async_trait]
pub trait WriteEnrichmentStrategy: Send + Sync {
    async fn persist(
        &self,
        tree: &RepositoryTree,
        items: &mut [Row],
        mode: WriteMode,
    ) -> Result<()>;
}
