namespace Datasources;

public sealed class RepositoryTreeNode(
    object value,
    string propertyName,
    IRepositoryTreeNode? parent = null,
    Enrichment? enrichment = null) : IRepositoryTreeNode
{
    public object Value { get; } = value;
    public string PropertyName { get; } = propertyName;
    public IRepositoryTreeNode? Parent { get; } = parent;
    public Enrichment? Enrichment { get; } = enrichment;
}
