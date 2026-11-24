namespace Datasources;

public class DefaultReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T> where T : new()
{
    const int DefaultBatchSize = 500;

    protected virtual int BatchSize => DefaultBatchSize;

    public virtual Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null) =>
        enrichment.Type switch
        {
            EnrichmentPropertyType.List => ApplyList(via, enrichment, parents, sources),
            EnrichmentPropertyType.Object => ApplyObject(via, enrichment, parents, sources),
            EnrichmentPropertyType.Dictionary => ApplyDictionary(via, enrichment, parents, sources),
            _ => throw new DatasourceException($"Unsupported enrichment type \"{enrichment.Type}\"")
        };

    public virtual async Task ApplyLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        IReadonlyRepository<TLink> link,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var parentKeys = DistinctKeys(parents, sources, enrichment);
        var links = await LoadByKeys(link, association.Parent, parentKeys);
        var relatedIds = new List<Value>();
        var seen = new HashSet<string>();
        foreach (var row in links)
        {
            if (Get(row!, association.Related) is not { } id) continue;
            if (seen.Add(KeyOf(id)))
                relatedIds.Add(Value.From(id));
        }
        var relatedById = new Dictionary<string, TRelated>();
        foreach (var item in await LoadByKeys(via, enrichment.RelatedKey, relatedIds))
        {
            if (Get(item!, enrichment.RelatedKey) is { } id)
                relatedById.TryAdd(KeyOf(id), item);
        }
        var linksByParent = new Dictionary<string, List<TLink>>();
        foreach (var row in links)
        {
            if (Get(row!, association.Parent) is not { } parentKey) continue;
            var cacheKey = KeyOf(parentKey);
            if (!linksByParent.TryGetValue(cacheKey, out var list))
                linksByParent[cacheKey] = list = [];
            list.Add(row);
        }
        for (var i = 0; i < parents.Count; i++)
        {
            var key = SourceKey(parents[i], enrichment, At(sources, i));
            if (key is null) continue;
            var related = new List<TRelated>();
            if (linksByParent.TryGetValue(KeyOf(key), out var rows))
            {
                foreach (var row in rows)
                {
                    if (Get(row!, association.Related) is not { } id) continue;
                    if (relatedById.TryGetValue(KeyOf(id), out var entity))
                        related.Add(entity);
                }
            }
            Set(parents[i], enrichment.DestinationProperty, related);
        }
    }

    protected virtual async Task ApplyObject<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var index = new Dictionary<string, TRelated>();
        foreach (var item in await LoadByKeys(via, enrichment.RelatedKey, DistinctKeys(parents, sources, enrichment)))
        {
            var cacheKey = RowMatchKey(item!, enrichment);
            if (cacheKey is not null)
                index.TryAdd(cacheKey, item);
        }
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = MatchKey(parent, enrichment, At(sources, i));
            if (key is null || !index.TryGetValue(key, out var related))
                continue;
            Set(
                parent,
                enrichment.DestinationProperty,
                enrichment.RelatedProperty is { } select ? Get(related!, select) : related);
        }
    }

    protected virtual async Task ApplyList<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var index = Group(await LoadByKeys(via, enrichment.RelatedKey, DistinctKeys(parents, sources, enrichment)), enrichment);
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = MatchKey(parent, enrichment, At(sources, i));
            if (key is null) continue;
            Set(parent, enrichment.DestinationProperty, index.GetValueOrDefault(key) ?? []);
        }
    }

    protected virtual async Task ApplyDictionary<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var index = Group(await LoadByKeys(via, enrichment.RelatedKey, DistinctKeys(parents, sources, enrichment)), enrichment);
        var schema = via.Schema;
        var mapKey = EnrichmentCells.MapKey(enrichment);
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = MatchKey(parent, enrichment, At(sources, i));
            if (key is null) continue;
            var map = EnrichmentCells.MapOf(parent, enrichment.DestinationProperty);
            if (index.TryGetValue(key, out var items))
            {
                foreach (var item in items)
                {
                    if (Get(item!, mapKey) is not { } entry) continue;
                    map[entry] = EnrichmentCells.DictionaryValue(item, enrichment, schema, map);
                }
            }
            Set(parent, enrichment.DestinationProperty, map);
        }
    }

    protected async Task<List<TItem>> LoadByKeys<TItem>(
        IReadonlyRepository<TItem> via,
        string column,
        IReadOnlyList<Value> keys)
    {
        var related = new List<TItem>();
        if (keys.Count == 0) return related;
        foreach (var chunk in keys.Chunk(BatchSize))
            await foreach (var item in via.Read(new Where(new Operator.In(column, chunk))))
                related.Add(item);
        return related;
    }

    protected static List<Value> DistinctKeys(IList<T> parents, IReadOnlyList<Row?>? sources, Enrichment enrichment)
    {
        var keys = new List<Value>();
        var seen = new HashSet<string>();
        for (var i = 0; i < parents.Count; i++)
        {
            var key = SourceKey(parents[i], enrichment, At(sources, i));
            if (key is null) continue;
            if (seen.Add(KeyOf(key)))
                keys.Add(Value.From(key));
        }
        return keys;
    }

    protected static Dictionary<string, List<TRelated>> Group<TRelated>(IReadOnlyList<TRelated> items, Enrichment enrichment)
    {
        var index = new Dictionary<string, List<TRelated>>();
        foreach (var item in items)
        {
            var cacheKey = RowMatchKey(item!, enrichment);
            if (cacheKey is null) continue;
            if (!index.TryGetValue(cacheKey, out var list))
                index[cacheKey] = list = [];
            list.Add(item);
        }
        return index;
    }

    internal static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;

    protected static object? SourceValue<TParent>(TParent parent, string name, Row? source) =>
        source is not null && source.TryGetValue(name, out var cell) && cell is not Value.Null
            ? cell.ToObject()
            : Get(parent, name);

    protected static object? SourceKey<TParent>(TParent parent, Enrichment enrichment, Row? source) =>
        SourceValue(parent, enrichment.SourceProperty, source);

    internal static string? MatchKey<TParent>(TParent parent, Enrichment enrichment, Row? source)
    {
        var parts = new List<string>();
        foreach (var pair in enrichment.Pairs())
        {
            var key = SourceValue(parent, pair.SourceProperty, source);
            if (key is null) return null;
            parts.Add(KeyOf(key));
        }
        return string.Join('\0', parts);
    }

    internal static string? RowMatchKey<TItem>(TItem row, Enrichment enrichment)
    {
        var parts = new List<string>();
        foreach (var pair in enrichment.Pairs())
        {
            var key = Get(row, pair.RelatedKey);
            if (key is null) return null;
            parts.Add(KeyOf(key));
        }
        return string.Join('\0', parts);
    }

    protected static Where? MatchWhere<TParent>(TParent parent, Enrichment enrichment, Row? source)
    {
        Condition? condition = null;
        foreach (var pair in enrichment.Pairs())
        {
            var key = SourceValue(parent, pair.SourceProperty, source);
            if (key is null) return null;
            Condition equals = new Operator.Equals(pair.RelatedKey, Value.From(key));
            condition = condition is null ? equals : new Operator.And(condition, equals);
        }
        return condition is null ? null : new Where(condition);
    }

    internal static string KeyOf(object? key)
    {
        if (key is null) return "";
        if (key is Value value)
            return KeyOf(value.ToObject());
        if (key is string or bool or byte or sbyte or short or ushort or int or uint or long or ulong or float or double or decimal or Guid or DateTime)
            return key.ToString() ?? "";
        var id = EnrichmentCells.Get(key, "Id") ?? EnrichmentCells.Get(key, "id");
        return id is null ? key.ToString() ?? "" : KeyOf(id);
    }

    protected static object? Get<TItem>(TItem obj, string name) => EnrichmentCells.Get(obj, name);

    protected static void Set<TItem>(TItem obj, string name, object? value) =>
        EnrichmentCells.Set(obj, name, value);
}
