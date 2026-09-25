namespace Datasources;

public sealed class DefaultReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T>, ITrackedApply where T : new()
{
    public Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents) =>
        ((ITrackedApply)this).ApplyTracked(via, enrichment, parents, null);

    Task ITrackedApply.ApplyTracked<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents,
        IReadOnlyList<Row?>? sources) =>
        enrichment.Many
            ? ApplyMany(via, enrichment, parents, sources)
            : ApplyLookup(via, enrichment, parents, sources);

    static async Task ApplyLookup<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents,
        IReadOnlyList<Row?>? sources)
    {
        var cache = new Dictionary<string, TRelated?>();
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = RepositoryTreeNode.SourceKey(parent, enrichment.From, At(sources, i));
            if (key is null) continue;
            var cacheKey = RepositoryTreeNode.KeyOf(key);
            if (!cache.TryGetValue(cacheKey, out var related))
            {
                related = await via.ReadSingle(new Where(new Operator.Equals(enrichment.By, Value.From(key))));
                cache[cacheKey] = related;
            }
            if (related is null) continue;
            RepositoryTreeNode.Set(
                parent,
                enrichment.Into,
                enrichment.Select is { } select ? RepositoryTreeNode.Get(related, select) : related);
        }
    }

    static async Task ApplyMany<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents,
        IReadOnlyList<Row?>? sources)
    {
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = RepositoryTreeNode.SourceKey(parent, enrichment.From, At(sources, i));
            if (key is null) continue;
            var related = new List<TRelated>();
            await foreach (var item in via.Read(new Where(new Operator.Equals(enrichment.By, Value.From(key)))))
                related.Add(item);
            RepositoryTreeNode.Set(parent, enrichment.Into, related);
        }
    }

    static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;
}
