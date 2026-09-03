namespace Datasources;

public sealed class RepositoryTree : IRepositoryTree
{
    public RepositoryTree(IRepositoryTreeNode parent) => Parent = parent;
    public IRepositoryTreeNode Parent { get; }
}
