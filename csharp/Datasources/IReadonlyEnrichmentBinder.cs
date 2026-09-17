namespace Datasources;

public interface IReadonlyEnrichmentBinder
{
    Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents);
}
