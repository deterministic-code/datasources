namespace Datasources;

internal abstract class PersistEnrichment<T>
{
    internal abstract Task Write(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        T parent);

    internal abstract Task WriteBulk(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        WriteMode mode,
        IReadOnlyList<T> parents);
}

internal sealed class PersistEnrichment<T, TChild> : PersistEnrichment<T> where TChild : new()
{
    readonly ICRUDRepository<TChild> repository;

    internal PersistEnrichment(ICRUDRepository<TChild> repository) => this.repository = repository;

    internal override Task Write(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        T parent) =>
        mode switch
        {
            WriteMode.Create => create.CreateChildren(repository, enrichment, parent),
            WriteMode.Update => update.UpdateChildren(repository, enrichment, parent),
            WriteMode.Delete => delete.DeleteChildren(repository, enrichment, parent, _ => Task.CompletedTask),
            _ => throw new DatasourceException("Unknown persist mode")
        };

    internal override async Task WriteBulk(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        WriteMode mode,
        IReadOnlyList<T> parents)
    {
        var children = new List<TChild>();
        foreach (var parent in parents)
            children.AddRange(create.CollectChildren(repository, enrichment, parent));
        if (children.Count == 0) return;
        await (repository is BulkCRUDRepository<TChild> bulk
            ? mode switch
            {
                WriteMode.Create => Run(bulk.Create(children)),
                WriteMode.Update => Run(bulk.Update(children)),
                _ => throw new DatasourceException("Unknown persist mode")
            }
            : mode switch
            {
                WriteMode.Create => Each(children, child => repository.Create(child)),
                WriteMode.Update => Each(children, child => repository.Update(child)),
                _ => throw new DatasourceException("Unknown persist mode")
            });
    }

    static async Task Each(IReadOnlyList<TChild> children, Func<TChild, Task> write)
    {
        foreach (var child in children)
            await write(child);
    }

    static async Task Run(Task write) => await write;
}
