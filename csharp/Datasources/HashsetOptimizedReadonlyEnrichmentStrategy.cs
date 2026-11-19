namespace Datasources;

public sealed class HashsetOptimizedReadonlyEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
{
    protected override async Task ApplyObject<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var keys = Keys(parents, sources, enrichment);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, TRelated>();
        await foreach (var item in via.Read(new Where(new Operator.In(enrichment.RelatedKey, keys.Values.ToList()))))
        {
            var cacheKey = RowMatchKey(item!, enrichment);
            if (cacheKey is not null)
                index[cacheKey] = item;
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

    protected override async Task ApplyList<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var keys = Keys(parents, sources, enrichment);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, List<TRelated>>();
        await foreach (var item in via.Read(new Where(new Operator.In(enrichment.RelatedKey, keys.Values.ToList()))))
        {
            var cacheKey = RowMatchKey(item!, enrichment);
            if (cacheKey is null) continue;
            if (!index.TryGetValue(cacheKey, out var list))
                index[cacheKey] = list = [];
            list.Add(item);
        }
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = MatchKey(parent, enrichment, At(sources, i));
            if (key is null) continue;
            Set(
                parent,
                enrichment.DestinationProperty,
                index.GetValueOrDefault(key) ?? []);
        }
    }

    static Dictionary<string, Value> Keys(IList<T> parents, IReadOnlyList<Row?>? sources, Enrichment enrichment)
    {
        var keys = new Dictionary<string, Value>();
        for (var i = 0; i < parents.Count; i++)
        {
            var key = SourceKey(parents[i], enrichment, At(sources, i));
            if (key is not null)
                keys.TryAdd(KeyOf(key), Value.From(key));
        }
        return keys;
    }
}
