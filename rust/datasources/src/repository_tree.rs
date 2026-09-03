use crate::repository_tree_node::RepositoryTreeNode;

pub struct RepositoryTree {
    pub parent: RepositoryTreeNode,
}

impl RepositoryTree {
    pub fn new(parent: RepositoryTreeNode) -> Self {
        Self { parent }
    }
}
