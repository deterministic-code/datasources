namespace Datasources;

public interface IRepositoryTreeNode
{
    object Value { get; }
    string PropertyName { get; }
    IRepositoryTreeNode? Parent { get; }
    Enrichment? Enrichment { get; }
}
