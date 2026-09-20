use crate::query::Query;
use crate::repository::ReadonlyRepository;
use crate::repository_tree::RepositoryTree;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use crate::where_trait::Where;
use async_trait::async_trait;

#[async_trait]
pub trait ReadonlyEnrichmentStrategy: Send + Sync {
    async fn read(
        &self,
        inner: &dyn ReadonlyRepository,
        tree: &RepositoryTree,
        query: Option<&dyn Query>,
    ) -> Result<Vec<Row>>;

    async fn read_where(
        &self,
        inner: &dyn ReadonlyRepository,
        tree: &RepositoryTree,
        where_clause: &dyn Where,
    ) -> Result<Vec<Row>>;

    async fn enrich(&self, tree: &RepositoryTree, items: &mut [Row]) -> Result<()>;

    async fn apply(&self, node: &RepositoryTreeNode, parents: &mut [Row]) -> Result<()>;
}
