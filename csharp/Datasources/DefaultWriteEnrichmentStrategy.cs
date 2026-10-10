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

    public Task PersistBulk(IRepositoryTree tree, IList<T> items, WriteMode mode) =>
        mode is WriteMode.Delete ? Persist(tree, items, mode) : WalkBulk(tree.Parent.Value, items, mode);

    protected virtual bool SkipUnloaded => false;

    async Task Walk(object host, IList<T> parents, WriteMode mode)
    {
        if (host is not IEnrichmentHost enriched) return;
        foreach (var entry in enriched.Enrichments)
            await PersistEntry(entry, parents, mode);
    }

    async Task WalkBulk(object host, IList<T> parents, WriteMode mode)
    {
        if (host is not IEnrichmentHost enriched) return;
        foreach (var entry in enriched.Enrichments)
            await PersistEntryBulk(entry, parents, mode);
    }

    async Task PersistEntryBulk(StoredEnrichment entry, IList<T> parents, WriteMode mode)
    {
        if (entry.Enrichment.Type is EnrichmentPropertyType.Custom)
            throw new DatasourceException($"Unsupported enrichment type \"{entry.Enrichment.Type}\"");
        var stored = (StoredEnrichment<T>)entry;
        if (entry.Enrichment is { Association: { } } enrichment)
        {
            foreach (var parent in parents)
            {
                if (Skip(parent, enrichment.DestinationProperty, mode)) continue;
                await stored.WriteLinked(create, update, delete, mode, parent);
            }
            return;
        }
        if (entry.Enrichment is { Type: EnrichmentPropertyType.List, RelatedProperty: not null })
            return;
        if (entry.Enrichment.Type is not (EnrichmentPropertyType.List or EnrichmentPropertyType.Dictionary))
            return;
        var ready = parents.Where(parent => !Skip(parent, entry.Enrichment.DestinationProperty, mode)).ToList();
        if (ready.Count == 0) return;
        await stored.WriteChildrenBulk(create, update, mode, ready);
    }

    async Task PersistEntry(StoredEnrichment entry, IList<T> parents, WriteMode mode)
    {
        if (entry.Enrichment.Type is EnrichmentPropertyType.Custom)
            throw new DatasourceException($"Unsupported enrichment type \"{entry.Enrichment.Type}\"");
        var stored = (StoredEnrichment<T>)entry;
        if (entry.Enrichment is { Association: { } } enrichment)
        {
            foreach (var parent in parents)
            {
                if (Skip(parent, enrichment.DestinationProperty, mode)) continue;
                await stored.WriteLinked(create, update, delete, mode, parent);
            }
            return;
        }
        if (entry.Enrichment is { Type: EnrichmentPropertyType.List, RelatedProperty: not null })
            return;
        if (entry.Enrichment.Type is not (EnrichmentPropertyType.List or EnrichmentPropertyType.Dictionary))
            return;
        var children = entry.Enrichment;
        foreach (var parent in parents)
        {
            if (Skip(parent, children.DestinationProperty, mode)) continue;
            await stored.WriteChildren(create, update, delete, mode, parent);
        }
    }

    bool Skip(T parent, string into, WriteMode mode) =>
        SkipUnloaded && mode is not WriteMode.Delete && parent!.GetType().GetProperty(into)?.GetValue(parent) is null;
}
