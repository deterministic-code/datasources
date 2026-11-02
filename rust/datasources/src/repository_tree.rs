use crate::entity::Entity;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::row::Row;

pub struct RepositoryTree<T: Entity = Row> {
    pub parent: RepositoryTreeNode<T>,
}

impl<T: Entity> RepositoryTree<T> {
    pub fn new(parent: RepositoryTreeNode<T>) -> Self {
        Self { parent }
    }
}
