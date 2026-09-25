namespace Datasources;

public interface IRepositoryTreeNode
{
    object Value { get; }
    string PropertyName { get; }
    IRepositoryTreeNode? Parent { get; }
    IReadOnlyList<IRepositoryTreeNode> Items { get; }
    Enrichment? Enrichment { get; }
    IRepositoryTreeNode Include<TRelated>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        string? select = null,
        bool many = false);
    IRepositoryTreeNode IncludeLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        ICRUDRepository<TLink> link,
        string parent,
        string related);
    Task EnrichFrom(IReadOnlyList<object> parents);
}
