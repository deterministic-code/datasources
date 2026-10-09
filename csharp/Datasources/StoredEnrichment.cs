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

    internal StoredEnrichment(Enrichment enrichment, Func<IList<T>, IReadOnlyList<Row?>?, Task> apply) :
        base(enrichment) =>
        Apply = apply;

    internal Func<IList<T>, IReadOnlyList<Row?>?, Task> Apply { get; private set; }

    internal void AttachPersist<TChild>(ICRUDRepository<TChild> repository)
        where TChild : new() =>
        persistWrite = (create, update, delete, mode, parent) => mode switch
        {
            WriteMode.Create => create.CreateChildren(repository, Enrichment, parent),
            WriteMode.Update => update.UpdateChildren(repository, Enrichment, parent),
            WriteMode.Delete => delete.DeleteChildren(repository, Enrichment, parent, _ => Task.CompletedTask),
            _ => throw new DatasourceException("Unknown persist mode")
        };

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
