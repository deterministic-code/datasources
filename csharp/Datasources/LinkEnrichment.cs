namespace Datasources;

internal abstract class LinkEnrichment<T>
{
    internal abstract Task Write(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        T parent);
}

internal sealed class LinkEnrichment<T, TLink> : LinkEnrichment<T> where TLink : new()
{
    readonly ICRUDRepository<TLink> link;

    internal LinkEnrichment(ICRUDRepository<TLink> link) => this.link = link;

    internal override Task Write(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        T parent) =>
        mode switch
        {
            WriteMode.Create => create.CreateLinked(link, enrichment, parent),
            WriteMode.Update => update.UpdateLinked(link, enrichment, parent),
            WriteMode.Delete => delete.DeleteLinked(link, enrichment, parent),
            _ => throw new DatasourceException("Unknown persist mode")
        };
}
