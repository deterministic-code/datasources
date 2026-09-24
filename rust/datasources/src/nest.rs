use crate::repository_tree_node::RepositoryTreeNode;

pub trait Nest {
    fn apply(self, node: &mut RepositoryTreeNode);
}

impl Nest for () {
    fn apply(self, _node: &mut RepositoryTreeNode) {}
}

impl<F> Nest for F
where
    F: FnOnce(&mut RepositoryTreeNode),
{
    fn apply(self, node: &mut RepositoryTreeNode) {
        self(node);
    }
}
