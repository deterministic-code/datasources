use crate::read_input::ReadInput;
use crate::repository::IReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait ReadonlyEnrichmentStrategy: Send + Sync {
    async fn read(
        &self,
        inner: &dyn IReadonlyRepository,
        tree: &RepositoryTree,
        query: Option<&dyn ReadInput>,
    ) -> Result<Vec<Row>>;

    async fn enrich(&self, tree: &RepositoryTree, items: &mut Vec<Row>) -> Result<()>;

    async fn apply(&self, node: &RepositoryTreeNode, parents: &mut [Row]) -> Result<()>;
}
