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

    internal abstract Task WriteMany(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        IReadOnlyList<T> parents);

    internal abstract Task WriteBulk(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
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
                    await create.CreateChildren(repository, enrichment, parent);
                break;
            case WriteMode.Update:
                await update.UpdateChildrenBatch(repository, enrichment, parents);
                break;
            case WriteMode.Delete:
                await delete.DeleteChildrenBatch(repository, enrichment, parents, _ => Task.CompletedTask);
                break;
            default:
                throw new DatasourceException("Unknown persist mode");
        }
    }

    internal override async Task WriteBulk(
        Enrichment enrichment,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        WriteMode mode,
        IReadOnlyList<T> parents)
    {
        var children = new List<TChild>();
        foreach (var parent in parents)
            children.AddRange(create.CollectChildren(repository, enrichment, parent));
        if (children.Count == 0) return;
        if (repository is BulkCRUDRepository<TChild> bulk)
        {
            await (mode switch
            {
                WriteMode.Create => Run(bulk.Create(children)),
                WriteMode.Update => Run(bulk.Update(children)),
                _ => throw new DatasourceException("Unknown persist mode")
            });
            return;
        }
        if (mode is WriteMode.Create)
        {
            await Each(children, repository.Create);
            return;
        }
        if (mode is WriteMode.Update)
        {
            await update.UpdateChildrenBatch(repository, enrichment, parents);
            return;
        }
        throw new DatasourceException("Unknown persist mode");
    }

    static async Task Each(IReadOnlyList<TChild> children, Func<TChild, Task> write)
    {
        foreach (var child in children)
            await write(child);
    }

    static async Task Run(Task write) => await write;
}
