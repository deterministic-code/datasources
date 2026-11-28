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

    internal static IEnumerable<string> ObjectSources(IReadOnlyList<StoredEnrichment> entries) =>
        entries
            .Where(entry => entry.Enrichment.Type is EnrichmentPropertyType.Object && entry.Enrichment.Association is null)
            .Select(entry => entry.Enrichment.SourceProperty);

    internal static IReadOnlySet<string> Sources(IReadOnlyList<StoredEnrichment> entries, Func<string, string> column) =>
        ObjectSources(entries).SelectMany(source => new[] { source, column(source) }).ToHashSet();

    internal static bool Destination(IReadOnlyList<StoredEnrichment> entries, string name) =>
        entries.Any(entry => entry.Enrichment.DestinationProperty == name);

    internal async Task Lookup(
        IReadOnlyList<StoredEnrichment> entries,
        IList<T> items,
        Row record,
        Schema schema,
        Func<string, string> column)
    {
        foreach (var source in ObjectSources(entries))
        {
            var mapped = column(source);
            if (Missing(Cell(record, source)) && !Missing(Cell(record, mapped)))
                record[source] = record[mapped];
        }
        await ResolveLookups(entries, items, record, schema);
        foreach (var source in ObjectSources(entries))
        {
            if (Missing(Cell(record, source))) continue;
            var mapped = column(source);
            if (mapped != source)
                record.Remove(mapped);
        }
    }

    static Value? Cell(Row record, string name) =>
        record.TryGetValue(name, out var value) ? value : null;

    protected static string Text(Value value) => EnrichmentCells.Text(value);

    protected static IReadOnlyList<TItem> Payload<TParent, TItem>(TParent parent, string into) =>
        EnrichmentCells.Payload<TParent, TItem>(parent, into);

    internal async Task ResolveLookups(IReadOnlyList<StoredEnrichment> entries, IList<T> items, Row record, Schema schema)
    {
        foreach (var entry in entries)
            if (entry is StoredEnrichment<T> { Enrichment: { Association: not null, RelatedProperty: { } linkedBy } } linked)
                await ResolveLinked(linked, items, linkedBy);
        foreach (var item in items)
        foreach (var entry in entries)
        {
            if (entry.Enrichment.Type is not EnrichmentPropertyType.Object) continue;
            if (entry.Enrichment.Association is not null) continue;
            if (entry.Enrichment.RelatedProperty is not null) continue;
            var source = entry.Enrichment.SourceProperty;
            if (record.TryGetValue(source, out var present) && !Missing(present)) continue;
            if (Get(item, entry.Enrichment.DestinationProperty) is not { } nested) continue;
            var nestedId = Value.From(Get(nested, entry.Enrichment.RelatedKey));
            if (Missing(nestedId)) continue;
            Set(record, source, nestedId);
        }
        foreach (var item in items)
        foreach (var entry in entries)
        {
            if (entry is not StoredEnrichment<T> stored || entry.Enrichment.RelatedProperty is not { } related)
                continue;
            if (entry.Enrichment.Association is not null)
                continue;
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

    /// <summary>
    /// Fills the missing related keys of every item's linked rows from their names, with one batched
    /// lookup per enrichment instead of one per row.
    /// </summary>
    static async Task ResolveLinked(StoredEnrichment<T> stored, IList<T> items, string related)
    {
        var enrichment = stored.Enrichment;
        var rows = items
            .SelectMany(item => Payload<T, object>(item, enrichment.DestinationProperty))
            .Where(row => Missing(Value.From(Get(row, enrichment.RelatedKey))))
            .Select(row => (Row: row, Name: Value.From(Get(row, related))))
            .Where(pair => !Missing(pair.Name))
            .ToList();
        if (rows.Count == 0) return;
        var names = new List<Value>();
        var seen = new HashSet<string>();
        foreach (var pair in rows)
            if (seen.Add(DefaultReadonlyEnrichmentStrategy<object>.KeyOf(pair.Name)))
                names.Add(pair.Name);
        var found = await stored.Read.LookupBatch(related, names, enrichment.RelatedKey);
        foreach (var (row, name) in rows)
        {
            if (!found.TryGetValue(DefaultReadonlyEnrichmentStrategy<object>.KeyOf(name), out var id) || Missing(id))
                throw new DatasourceException($"Unknown {enrichment.DestinationProperty} \"{name.ToObject()}\"");
            Set(row, enrichment.RelatedKey, id.ToObject());
        }
    }

    public IReadOnlyList<TChild> CollectChildren<TChild>(ICRUDRepository<TChild> writer, Enrichment enrichment, T parent)
        where TChild : new()
    {
        var children = Children(writer, enrichment, parent);
        if (children.Count == 0) return children;
        foreach (var pair in enrichment.Pairs())
        {
            var key = Required(parent, pair.SourceProperty, enrichment.DestinationProperty);
            foreach (var child in children)
                Stamp(child, pair.RelatedKey, key.ToObject());
        }
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
