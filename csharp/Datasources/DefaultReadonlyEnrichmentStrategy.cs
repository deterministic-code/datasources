namespace Datasources;

public sealed class DefaultReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T>
{
    public async IAsyncEnumerable<T> Read(
        IReadonlyRepository<T> inner,
        IRepositoryTree tree,
        IStatement? query)
    {
        var items = new List<T>();
        await foreach (var item in inner.Read(query))
            items.Add(item);
        await Enrich(tree, items);
        foreach (var item in items)
            yield return item;
    }

    public Task Enrich(IRepositoryTree tree, IList<T> items) =>
        tree.Parent.EnrichFrom(items.Cast<object>().ToList());

    public Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents) =>
        enrichment.Many ? ApplyMany(via, enrichment, parents) : ApplyLookup(via, enrichment, parents);

    static async Task ApplyLookup<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents)
    {
        var cache = new Dictionary<string, TRelated?>();
        foreach (var parent in parents)
        {
            var key = RepositoryTreeNode.Get(parent, enrichment.From);
            if (key is null) continue;
            var cacheKey = RepositoryTreeNode.KeyOf(key);
            if (!cache.TryGetValue(cacheKey, out var related))
            {
                related = await via.ReadSingle(new WhereClause(new Operator.Equals(enrichment.By, Value.From(key))));
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
        IReadOnlyList<object> parents)
    {
        foreach (var parent in parents)
        {
            var key = RepositoryTreeNode.Get(parent, enrichment.From);
            if (key is null) continue;
            var related = new List<TRelated>();
            await foreach (var item in via.Read(new WhereClause(new Operator.Equals(enrichment.By, Value.From(key)))))
                related.Add(item);
            RepositoryTreeNode.Set(parent, enrichment.Into, related);
        }
    }
}
