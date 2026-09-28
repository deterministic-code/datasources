using System.Collections;

namespace Datasources;

public class DefaultWriteEnrichmentStrategy<T> : IWriteEnrichmentStrategy<T>
{
    readonly DeleteEnrichmentStrategy<T> delete;
    readonly UpdateEnrichmentStrategy<T> update;

    public DefaultWriteEnrichmentStrategy(
        DeleteEnrichmentStrategy<T>? delete = null,
        UpdateEnrichmentStrategy<T>? update = null,
        bool useTransaction = false)
    {
        this.delete = delete ?? new CascadeDeleteEnrichmentStrategy<T>();
        this.update = update ?? new DefaultUpdateEnrichmentStrategy<T>();
        UseTransaction = useTransaction;
    }

    public bool UseTransaction { get; }

    public Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode) =>
        Walk(tree.Parent.Value, [.. items.Cast<object>()], mode);

    protected virtual bool SkipUnloaded => false;

    async Task Walk(object host, IReadOnlyList<object> parents, WriteMode mode)
    {
        if (host is not IEnrichmentHost enriched) return;
        foreach (var entry in enriched.Enrichments)
            await PersistEntry(entry, parents, mode);
    }

    async Task PersistEntry(StoredEnrichment entry, IReadOnlyList<object> parents, WriteMode mode)
    {
        if (entry.Enrichment is { Association: { } association } enrichment)
        {
            var writer = entry.Link
                ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{enrichment.Into}\"");
            foreach (var parent in parents)
            {
                if (Skip(parent, enrichment.Into, mode)) continue;
                await (mode switch
                {
                    WriteMode.Create => update.CreateLinks(writer, enrichment, association, parent),
                    WriteMode.Update => update.UpdateLinked(entry, enrichment, parent),
                    WriteMode.Delete => delete.DeleteLinked(entry, enrichment, parent),
                    _ => throw new DatasourceException("Unknown persist mode")
                });
            }
            return;
        }
        if (entry.Enrichment is not { Many: true, Select: null } owned)
            return;
        var childWriter = entry.Persist
            ?? throw new DatasourceException($"Deep write requires a CRUD repository for \"{owned.Into}\"");
        foreach (var parent in parents)
        {
            if (Skip(parent, owned.Into, mode)) continue;
            await (mode switch
            {
                WriteMode.Create => update.CreateChildren(childWriter, owned, parent),
                WriteMode.Update => update.UpdateOwned(entry, owned, parent),
                WriteMode.Delete => delete.DeleteOwned(entry, owned, parent, _ => Task.CompletedTask),
                _ => throw new DatasourceException("Unknown persist mode")
            });
        }
    }

    bool Skip(object parent, string into, WriteMode mode) =>
        SkipUnloaded && mode is not WriteMode.Delete && parent.GetType().GetProperty(into)?.GetValue(parent) is null;
}
