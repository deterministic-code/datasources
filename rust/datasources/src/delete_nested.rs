use crate::repository_tree_node::RepositoryTreeNode;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;

#[async_trait]
pub trait DeleteNested: Send + Sync {
    async fn delete_nested(&self, node: &RepositoryTreeNode, rows: &mut [Row]) -> Result<()>;
}
