namespace Datasources;

public sealed class HashsetOptimizedReadonlyEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
{
    protected override async Task ApplyLookup<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources)
    {
        var keys = Keys(parents, sources, enrichment.From);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, TRelated>();
        await foreach (var item in via.Read(new Where(new Operator.In(enrichment.By, keys.Values.ToList()))))
        {
            var by = Get(item!, enrichment.By);
            if (by is not null)
                index[KeyOf(by)] = item;
        }
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = SourceKey(parent, enrichment.From, At(sources, i));
            if (key is null || !index.TryGetValue(KeyOf(key), out var related))
                continue;
            Set(
                parent,
                enrichment.Into,
                enrichment.Select is { } select ? Get(related!, select) : related);
        }
    }

    protected override async Task ApplyMany<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources)
    {
        var keys = Keys(parents, sources, enrichment.From);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, List<TRelated>>();
        await foreach (var item in via.Read(new Where(new Operator.In(enrichment.By, keys.Values.ToList()))))
        {
            var by = Get(item!, enrichment.By);
            if (by is null) continue;
            var cacheKey = KeyOf(by);
            if (!index.TryGetValue(cacheKey, out var list))
                index[cacheKey] = list = [];
            list.Add(item);
        }
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = SourceKey(parent, enrichment.From, At(sources, i));
            if (key is null) continue;
            Set(
                parent,
                enrichment.Into,
                index.GetValueOrDefault(KeyOf(key)) ?? []);
        }
    }

    static Dictionary<string, Value> Keys(IList<object> parents, IReadOnlyList<Row?>? sources, string from)
    {
        var keys = new Dictionary<string, Value>();
        for (var i = 0; i < parents.Count; i++)
        {
            var key = SourceKey(parents[i], from, At(sources, i));
            if (key is not null)
                keys.TryAdd(KeyOf(key), Value.From(key));
        }
        return keys;
    }
}
