namespace Datasources;

public sealed class HashsetOptimizedReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T>, ITrackedApply where T : new()
{
    public async IAsyncEnumerable<T> Read(
        IReadonlyRepository<T> inner,
        IRepositoryTree tree,
        IStatement? query)
    {
        var items = new List<T>();
        var sources = new List<Row?>();
        await foreach (var (item, source) in ReadonlyRepository<T>.Tracked(inner, query))
        {
            items.Add(item);
            sources.Add(source);
        }
        await Enrich(tree, items, sources);
        foreach (var item in items)
            yield return item;
    }

    public Task Enrich(IRepositoryTree tree, IList<T> items, IReadOnlyList<Row?>? sources = null) =>
        ((RepositoryTreeNode)tree.Parent).EnrichFrom(items.Cast<object>().ToList(), sources);

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
        var keys = Keys(parents, sources, enrichment.From);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, TRelated>();
        await foreach (var item in via.Read(new WhereClause(new Operator.In(enrichment.By, keys.Values.ToList()))))
        {
            var by = RepositoryTreeNode.Get(item!, enrichment.By);
            if (by is not null)
                index[RepositoryTreeNode.KeyOf(by)] = item;
        }
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = RepositoryTreeNode.SourceKey(parent, enrichment.From, At(sources, i));
            if (key is null || !index.TryGetValue(RepositoryTreeNode.KeyOf(key), out var related))
                continue;
            RepositoryTreeNode.Set(
                parent,
                enrichment.Into,
                enrichment.Select is { } select ? RepositoryTreeNode.Get(related!, select) : related);
        }
    }

    static async Task ApplyMany<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents,
        IReadOnlyList<Row?>? sources)
    {
        var keys = Keys(parents, sources, enrichment.From);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, List<TRelated>>();
        await foreach (var item in via.Read(new WhereClause(new Operator.In(enrichment.By, keys.Values.ToList()))))
        {
            var by = RepositoryTreeNode.Get(item!, enrichment.By);
            if (by is null) continue;
            var cacheKey = RepositoryTreeNode.KeyOf(by);
            if (!index.TryGetValue(cacheKey, out var list))
                index[cacheKey] = list = [];
            list.Add(item);
        }
        for (var i = 0; i < parents.Count; i++)
        {
            var parent = parents[i];
            var key = RepositoryTreeNode.SourceKey(parent, enrichment.From, At(sources, i));
            if (key is null) continue;
            RepositoryTreeNode.Set(
                parent,
                enrichment.Into,
                index.GetValueOrDefault(RepositoryTreeNode.KeyOf(key)) ?? []);
        }
    }

    static Dictionary<string, Value> Keys(
        IReadOnlyList<object> parents,
        IReadOnlyList<Row?>? sources,
        string from)
    {
        var keys = new Dictionary<string, Value>();
        for (var i = 0; i < parents.Count; i++)
        {
            var key = RepositoryTreeNode.SourceKey(parents[i], from, At(sources, i));
            if (key is not null)
                keys.TryAdd(RepositoryTreeNode.KeyOf(key), Value.From(key));
        }
        return keys;
    }

    static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;
}
