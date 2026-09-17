using Datasources.Builders;

namespace Datasources;

public sealed class HashsetOptimizedReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T>
{
    public async IAsyncEnumerable<T> Get(
        IReadonlyRepository<T> inner,
        IRepositoryTree tree,
        IStatement? query)
    {
        var items = new List<T>();
        await foreach (var item in inner.Get(query))
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
        var keys = Keys(parents, enrichment.From);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, TRelated>();
        await foreach (var item in via.Get(WhereBuilder.In(enrichment.By, keys.Values.ToList())))
        {
            var by = RepositoryTreeNode.Get(item!, enrichment.By);
            if (by is not null)
                index[RepositoryTreeNode.KeyOf(by)] = item;
        }
        foreach (var parent in parents)
        {
            var key = RepositoryTreeNode.Get(parent, enrichment.From);
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
        IReadOnlyList<object> parents)
    {
        var keys = Keys(parents, enrichment.From);
        if (keys.Count == 0) return;
        var index = new Dictionary<string, List<TRelated>>();
        await foreach (var item in via.Get(WhereBuilder.In(enrichment.By, keys.Values.ToList())))
        {
            var by = RepositoryTreeNode.Get(item!, enrichment.By);
            if (by is null) continue;
            var cacheKey = RepositoryTreeNode.KeyOf(by);
            if (!index.TryGetValue(cacheKey, out var list))
                index[cacheKey] = list = [];
            list.Add(item);
        }
        foreach (var parent in parents)
        {
            var key = RepositoryTreeNode.Get(parent, enrichment.From);
            if (key is null) continue;
            RepositoryTreeNode.Set(
                parent,
                enrichment.Into,
                index.GetValueOrDefault(RepositoryTreeNode.KeyOf(key)) ?? []);
        }
    }

    static Dictionary<string, Value> Keys(IReadOnlyList<object> parents, string from)
    {
        var keys = new Dictionary<string, Value>();
        foreach (var parent in parents)
        {
            var key = RepositoryTreeNode.Get(parent, from);
            if (key is not null)
                keys.TryAdd(RepositoryTreeNode.KeyOf(key), Value.From(key));
        }
        return keys;
    }
}
