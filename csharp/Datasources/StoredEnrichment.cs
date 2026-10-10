namespace Datasources;

internal class StoredEnrichment
{
    internal StoredEnrichment(Enrichment enrichment) => Enrichment = enrichment;

    public Enrichment Enrichment { get; }
}

internal sealed class StoredEnrichment<T> : StoredEnrichment
{
    Func<CreateEnrichmentStrategy<T>, UpdateEnrichmentStrategy<T>, DeleteEnrichmentStrategy<T>, WriteMode, T, Task>? persistWrite;
    Func<CreateEnrichmentStrategy<T>, UpdateEnrichmentStrategy<T>, DeleteEnrichmentStrategy<T>, WriteMode, T, Task>? linkWrite;
    Func<CreateEnrichmentStrategy<T>, UpdateEnrichmentStrategy<T>, WriteMode, IReadOnlyList<T>, Task>? persistBulkWrite;

    internal StoredEnrichment(Enrichment enrichment, Func<IList<T>, IReadOnlyList<Row?>?, Task> apply) :
        base(enrichment) =>
        Apply = apply;

    internal Func<IList<T>, IReadOnlyList<Row?>?, Task> Apply { get; private set; }

    internal void AttachPersist<TChild>(ICRUDRepository<TChild> repository)
        where TChild : new()
    {
        persistWrite = (create, update, delete, mode, parent) => mode switch
        {
            WriteMode.Create => create.CreateChildren(repository, Enrichment, parent),
            WriteMode.Update => update.UpdateChildren(repository, Enrichment, parent),
            WriteMode.Delete => delete.DeleteChildren(repository, Enrichment, parent, _ => Task.CompletedTask),
            _ => throw new DatasourceException("Unknown persist mode")
        };
        persistBulkWrite = (create, update, mode, parents) => WriteBulk(repository, create, update, mode, parents);
    }

    async Task WriteBulk<TChild>(
        ICRUDRepository<TChild> writer,
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        WriteMode mode,
        IReadOnlyList<T> parents)
        where TChild : new()
    {
        var children = new List<TChild>();
        foreach (var parent in parents)
            children.AddRange(create.CollectChildren(writer, Enrichment, parent));
        if (children.Count == 0) return;
        if (writer is BulkCRUDRepository<TChild> bulk)
        {
            if (mode is WriteMode.Create) await bulk.Create(children);
            else if (mode is WriteMode.Update) await bulk.Update(children);
            else throw new DatasourceException("Unknown persist mode");
            return;
        }
        foreach (var child in children)
            if (mode is WriteMode.Create) await writer.Create(child);
            else if (mode is WriteMode.Update) await writer.Update(child);
            else throw new DatasourceException("Unknown persist mode");
    }

    internal void AttachLink<TLink>(Func<IList<T>, IReadOnlyList<Row?>?, Task> apply, ICRUDRepository<TLink> link)
        where TLink : new()
    {
        Apply = apply;
        linkWrite = (create, update, delete, mode, parent) => mode switch
        {
            WriteMode.Create => create.CreateLinked(link, Enrichment, parent),
            WriteMode.Update => update.UpdateLinked(link, Enrichment, parent),
            WriteMode.Delete => delete.DeleteLinked(link, Enrichment, parent),
            _ => throw new DatasourceException("Unknown persist mode")
        };
    }

    internal Task WriteChildren(
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        T parent) =>
        persistWrite is { } write
            ? write(create, update, delete, mode, parent)
            : throw new DatasourceException($"Deep write requires a CRUD repository for \"{Enrichment.DestinationProperty}\"");

    internal Task WriteChildrenBulk(
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        WriteMode mode,
        IReadOnlyList<T> parents) =>
        persistBulkWrite is { } write
            ? write(create, update, mode, parents)
            : throw new DatasourceException($"Deep write requires a CRUD repository for \"{Enrichment.DestinationProperty}\"");

    internal Task WriteLinked(
        CreateEnrichmentStrategy<T> create,
        UpdateEnrichmentStrategy<T> update,
        DeleteEnrichmentStrategy<T> delete,
        WriteMode mode,
        T parent) =>
        linkWrite is { } write
            ? write(create, update, delete, mode, parent)
            : throw new DatasourceException($"Deep write requires a CRUD repository for \"{Enrichment.DestinationProperty}\"");
}
