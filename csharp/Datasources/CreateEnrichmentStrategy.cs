namespace Datasources;

public abstract class CreateEnrichmentStrategy<T>
{
    public abstract Task CreateChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new();

    public abstract Task CreateLinked<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new();

    protected static List<Value> RelatedIds<TParent>(TParent parent, Enrichment enrichment) =>
        EnrichmentCells.RelatedIds(parent, enrichment);

    protected static Value Required<TParent>(TParent parent, string name, string into) =>
        EnrichmentCells.Required(parent, name, into);

    protected static bool Missing(Value? value) => EnrichmentCells.Missing(value);

    protected static string Text(Value value) => EnrichmentCells.Text(value);

    protected static IReadOnlyList<TItem> Payload<TParent, TItem>(TParent parent, string into) =>
        EnrichmentCells.Payload<TParent, TItem>(parent, into);

    internal async Task ResolveLookups(IReadOnlyList<StoredEnrichment> entries, IList<T> items, Row record, Schema schema)
    {
        foreach (var item in items)
        foreach (var entry in entries)
        {
            if (entry is not StoredEnrichment<T> stored || entry.Enrichment.RelatedProperty is not { } related)
                continue;
            if (entry.Enrichment.Association is not null)
            {
                await ResolveLinked(stored, item, related);
                continue;
            }
            if (entry.Enrichment.Type is not EnrichmentPropertyType.Object) continue;
            var dest = entry.Enrichment.DestinationProperty;
            var source = entry.Enrichment.SourceProperty;
            if (record.TryGetValue(source, out var existing) && !Missing(existing)) continue;
            if (!record.TryGetValue(dest, out var name) || Missing(name))
            {
                if (schema.Columns.TryGetValue(dest, out var column) && !column.Nullable)
                    throw new DatasourceException($"Unknown {dest}");
                continue;
            }
            var id = await stored.Read.Lookup(related, name, entry.Enrichment.RelatedKey)
                ?? throw new DatasourceException($"Unknown {dest} \"{name.ToObject()}\"");
            Set(record, source, id);
        }
    }

    static async Task ResolveLinked(StoredEnrichment<T> stored, T item, string related)
    {
        foreach (var row in Payload<T, object>(item, stored.Enrichment.DestinationProperty))
        {
            if (!Missing(Value.From(Get(row, stored.Enrichment.RelatedKey)))) continue;
            var name = Value.From(Get(row, related));
            if (Missing(name)) continue;
            var id = await stored.Read.Lookup(related, name, stored.Enrichment.RelatedKey)
                ?? throw new DatasourceException($"Unknown {stored.Enrichment.DestinationProperty} \"{name.ToObject()}\"");
            Set(row, stored.Enrichment.RelatedKey, id.ToObject());
        }
    }

    public IReadOnlyList<TChild> CollectChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new()
    {
        var children = Children(writer, enrichment, parent);
        if (children.Count == 0) return children;
        var key = Required(parent, enrichment.SourceProperty, enrichment.DestinationProperty);
        foreach (var child in children)
            Stamp(child, enrichment.RelatedKey, key.ToObject());
        return children;
    }

    protected static IReadOnlyList<TChild> Children<TParent, TChild>(
        ICRUDRepository<TChild> writer,
        Enrichment enrichment,
        TParent parent)
        where TChild : new() =>
        EnrichmentCells.Children(writer, enrichment, parent);

    protected static void Stamp<TItem>(TItem related, string by, object? key) =>
        EnrichmentCells.Stamp(related, by, key);

    protected static object? Get<TItem>(TItem obj, string name) => EnrichmentCells.Get(obj, name);

    protected static void Set<TItem>(TItem obj, string name, object? value) =>
        EnrichmentCells.Set(obj, name, value);
}
