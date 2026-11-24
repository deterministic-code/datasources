namespace Datasources;

public class LoadLevelOrderReadonlyEnrichmentStrategy<T> : DefaultReadonlyEnrichmentStrategy<T> where T : new()
{
    const int DefaultBatchSize = 500;
    readonly int batchSize;

    public LoadLevelOrderReadonlyEnrichmentStrategy() : this(DefaultBatchSize) { }

    public LoadLevelOrderReadonlyEnrichmentStrategy(int batchSize) =>
        this.batchSize = batchSize < 1
            ? throw new DatasourceException("batchSize must be at least 1")
            : batchSize;

    public LoadLevelOrderReadonlyEnrichmentStrategy<T> WithBatchSize(int batchSize) => new(batchSize);

    protected override async Task ApplyObject<TRelated>(
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

    protected override async Task ApplyList<TRelated>(
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

    protected override async Task ApplyDictionary<TRelated>(
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

    public override async Task ApplyLinked<TRelated, TLink>(
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

    async Task<List<TItem>> LoadByKeys<TItem>(
        IReadonlyRepository<TItem> via,
        string column,
        IReadOnlyList<Value> keys)
    {
        var related = new List<TItem>();
        if (keys.Count == 0) return related;
        foreach (var chunk in keys.Chunk(batchSize))
            await foreach (var item in via.Read(new Where(new Operator.In(column, chunk))))
                related.Add(item);
        return related;
    }

    static List<Value> DistinctKeys(IList<T> parents, IReadOnlyList<Row?>? sources, Enrichment enrichment)
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

    static Dictionary<string, List<TRelated>> Group<TRelated>(IReadOnlyList<TRelated> items, Enrichment enrichment)
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
}
