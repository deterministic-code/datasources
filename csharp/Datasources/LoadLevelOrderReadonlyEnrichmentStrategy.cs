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

    protected override int BatchSize => batchSize;

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
}
