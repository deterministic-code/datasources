namespace Datasources;

public interface IEnriched<T>
{
    IEnriched<T> AddEnriched<TRelated>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        string? select = null,
        bool many = false,
        Action<IRepositoryTreeNode>? nest = null);
    Task Enrich(IList<T> items);
}
