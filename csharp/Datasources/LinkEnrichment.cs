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

    internal abstract Task WriteMany(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        IReadOnlyList<T> parents);
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

    internal override async Task WriteMany(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        IReadOnlyList<T> parents)
    {
        switch (mode)
        {
            case WriteMode.Create:
                foreach (var parent in parents)
                    await create.CreateLinked(link, enrichment, parent);
                break;
            case WriteMode.Update:
                await update.UpdateLinkedBatch(link, enrichment, parents);
                break;
            case WriteMode.Delete:
                await delete.DeleteLinkedBatch(link, enrichment, parents);
                break;
            default:
                throw new DatasourceException("Unknown persist mode");
        }
    }
}
