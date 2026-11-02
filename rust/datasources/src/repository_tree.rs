use crate::from_schema::FromSchema;
use crate::repository_tree_node::RepositoryTreeNode;
use crate::row::Row;

pub struct RepositoryTree<T: FromSchema = Row> {
    pub parent: RepositoryTreeNode<T>,
}

impl<T: FromSchema> RepositoryTree<T> {
    pub fn new(parent: RepositoryTreeNode<T>) -> Self {
        Self { parent }
    }
}
