namespace Datasources;

public interface IReadonlyEnrichmentStrategy<T>
{
    Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources = null);

    Task ApplyLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        ICRUDRepository<TLink> link,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources = null);
}
