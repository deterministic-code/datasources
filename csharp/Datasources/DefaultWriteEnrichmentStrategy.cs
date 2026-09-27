using System.Collections;
using System.Reflection;

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
                    WriteMode.Create => UpdateEnrichmentStrategy<T>.CreateLinks(writer, enrichment, association, parent),
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
                WriteMode.Create => UpdateEnrichmentStrategy<T>.CreateChildren(childWriter, owned, parent),
                WriteMode.Update => update.UpdateOwned(entry, owned, parent),
                WriteMode.Delete => delete.DeleteOwned(entry, owned, parent, _ => Task.CompletedTask),
                _ => throw new DatasourceException("Unknown persist mode")
            });
        }
    }

    bool Skip(object parent, string into, WriteMode mode) =>
        SkipUnloaded && mode is not WriteMode.Delete && UpdateEnrichmentStrategy<T>.Get(parent, into) is null;

    internal static object Strip(IEnrichmentHost host, object item)
    {
        var skip = host.Enrichments.Select(entry => entry.Enrichment.Into).ToHashSet();
        if (item is IDictionary<string, Value> row)
        {
            var copy = new Row(row);
            foreach (var name in skip)
                copy.Remove(name);
            return copy;
        }
        var dest = Activator.CreateInstance(item.GetType())!;
        foreach (var prop in item.GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance)
            .Where(prop => prop is { CanRead: true, CanWrite: true } && prop.GetIndexParameters().Length == 0))
        {
            if (skip.Contains(prop.Name))
            {
                if (!prop.PropertyType.IsValueType)
                    prop.SetValue(dest, null);
                continue;
            }
            prop.SetValue(dest, prop.GetValue(item));
        }
        return dest;
    }
}
