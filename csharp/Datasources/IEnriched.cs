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
        IReadonlyEnrichmentStrategy<T>? strategy = null);
    IEnriched<T> AddLinked<TRelated, TLink>(
        string into,
        string from,
        IReadonlyRepository<TRelated> via,
        string by,
        ICRUDRepository<TLink> link,
        string parent,
        string related,
        IReadonlyEnrichmentStrategy<T>? strategy = null);
    Task Enrich(IList<T> items);
}
