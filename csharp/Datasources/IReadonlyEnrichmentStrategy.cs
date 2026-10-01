namespace Datasources;

public interface IReadonlyEnrichmentStrategy<T>
{
    Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null);

    Task ApplyLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        IReadonlyRepository<TLink> link,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null);
}
