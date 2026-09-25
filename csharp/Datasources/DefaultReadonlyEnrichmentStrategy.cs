namespace Datasources;

public class DefaultReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T> where T : new()
{
    public virtual Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources = null) =>
        enrichment.Many
            ? ApplyMany(via, enrichment, parents, sources)
            : ApplyLookup(via, enrichment, parents, sources);

    public virtual async Task ApplyLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        ICRUDRepository<TLink> link,
        Enrichment enrichment,
        IList<object> parents,
        IReadOnlyList<Row?>? sources = null)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        for (var i = 0; i < parents.Count; i++)
        {
            var key = RepositoryTreeNode.SourceKey(parents[i], enrichment.From, At(sources, i));
            if (key is null) continue;
            var related = new List<TRelated>();
            await foreach (var row in link.Read(new Where(new Operator.Equals(association.Parent, Value.From(key)))))
            {
                if (RepositoryTreeNode.Get(row!, association.Related) is not { } id) continue;
                var entity = await via.ReadSingle(new Where(new Operator.Equals(enrichment.By, Value.From(id))));
                if (entity is not null)
                    related.Add(entity);
            }
            RepositoryTreeNode.Set(parents[i], enrichment.Into, related);
        }
    }

    protected virtual async Task ApplyLookup<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
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

    protected virtual async Task ApplyMany<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<object> parents,
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

    protected static Row? At(IReadOnlyList<Row?>? sources, int index) =>
        sources is not null && index < sources.Count ? sources[index] : null;
}
