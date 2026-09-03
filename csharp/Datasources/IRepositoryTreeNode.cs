namespace Datasources;

public interface IRepositoryTreeNode
{
    object Value { get; }
    string PropertyName { get; }
    IRepositoryTreeNode? Parent { get; }
    IReadOnlyList<IRepositoryTreeNode> Items { get; }
    IRepositoryTreeNode Include<TRelated>(
        IReadonlyRepository<TRelated> via,
        string into,
        string from,
        string by,
        string? select = null,
        bool many = false);
    Task EnrichFrom(IReadOnlyList<object> parents);
}
