namespace Datasources;

public class DefaultWriteEnrichmentStrategy<T> : IWriteEnrichmentStrategy<T>
{
    readonly CreateEnrichmentStrategy<T> create;
    readonly DeleteEnrichmentStrategy<T> delete;
    readonly UpdateEnrichmentStrategy<T> update;

    public DefaultWriteEnrichmentStrategy(
        DeleteEnrichmentStrategy<T>? delete = null,
        UpdateEnrichmentStrategy<T>? update = null,
        CreateEnrichmentStrategy<T>? create = null,
        bool useTransaction = false)
    {
        this.create = create ?? new DefaultCreateEnrichmentStrategy<T>();
        this.delete = delete ?? new CascadeDeleteEnrichmentStrategy<T>();
        this.update = update ?? new DefaultUpdateEnrichmentStrategy<T>();
        UseTransaction = useTransaction;
    }

    public bool UseTransaction { get; }

    public Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode) =>
        Walk(tree.Parent.Value, items, mode);

    protected virtual bool SkipUnloaded => false;

    async Task Walk(object host, IList<T> parents, WriteMode mode)
    {
        if (host is not IEnrichmentHost enriched) return;
        foreach (var entry in enriched.Enrichments)
            await PersistEntry(entry, parents, mode);
    }

    async Task PersistEntry(StoredEnrichment entry, IList<T> parents, WriteMode mode)
    {
        if (entry.Enrichment.Type is EnrichmentPropertyType.Custom)
            throw new DatasourceException($"Unsupported enrichment type \"{entry.Enrichment.Type}\"");
        if (entry.Enrichment is { Association: { } } enrichment)
        {
            var writer = entry.Link
                ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.DestinationProperty}\"");
            foreach (var parent in parents)
            {
                if (Skip(parent, enrichment.DestinationProperty, mode)) continue;
                await (mode switch
                {
                    WriteMode.Create => create.CreateLinked(writer, enrichment, parent),
                    WriteMode.Update => update.UpdateLinked(writer, enrichment, parent),
                    WriteMode.Delete => delete.DeleteLinked(writer, enrichment, parent),
                    _ => throw new DatasourceException("Unknown persist mode")
                });
            }
            return;
        }
        if (entry.Enrichment is { Type: EnrichmentPropertyType.List, RelatedProperty: not null })
            return;
        if (entry.Enrichment.Type is not (EnrichmentPropertyType.List or EnrichmentPropertyType.Dictionary))
            return;
        var owned = entry.Enrichment;
        var childWriter = entry.Persist
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{owned.DestinationProperty}\"");
        foreach (var parent in parents)
        {
            if (Skip(parent, owned.DestinationProperty, mode)) continue;
            await (mode switch
            {
                WriteMode.Create => create.CreateOwned(childWriter, owned, parent),
                WriteMode.Update => update.UpdateOwned(childWriter, owned, parent),
                WriteMode.Delete => delete.DeleteOwned(childWriter, owned, parent, _ => Task.CompletedTask),
                _ => throw new DatasourceException("Unknown persist mode")
            });
        }
    }

    bool Skip(T parent, string into, WriteMode mode) =>
        SkipUnloaded && mode is not WriteMode.Delete && parent!.GetType().GetProperty(into)?.GetValue(parent) is null;
}
