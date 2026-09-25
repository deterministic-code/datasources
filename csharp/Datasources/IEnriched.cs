namespace Datasources;

public interface IEnriched<T>
{
    IEnriched<T> AddEnriched<TRelated>(
        Enrichment enrichment,
        IReadonlyRepository<TRelated> via,
        IReadonlyEnrichmentStrategy<T>? strategy = null);
    IEnriched<T> AddLinked<TRelated, TLink>(
        Enrichment enrichment,
        IReadonlyRepository<TRelated> via,
        ICRUDRepository<TLink> link,
        IReadonlyEnrichmentStrategy<T>? strategy = null);
    Task Enrich(IList<T> items);
}
