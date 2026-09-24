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
    IEnriched<T> AddLinked<TRelated, TLink>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        ICRUDRepository<TLink> link,
        string parent,
        string related);
    Task Enrich(IList<T> items);
}
