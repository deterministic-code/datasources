namespace Datasources;

public sealed class TreeReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentStrategy<T> where T : new()
{
    public Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null) =>
        Fill(via, enrichment, parents, sources);

    public Task ApplyLinked<TRelated, TLink>(
        IReadonlyRepository<TRelated> via,
        IReadonlyRepository<TLink> link,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null) =>
        throw new DatasourceException("Tree enrichment does not use a junction");

    static async Task Fill<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var rows = new List<TRelated>();
        await foreach (var item in via.Read())
            rows.Add(item);
        var index = new Dictionary<string, List<TRelated>>();
        foreach (var item in rows)
        {
            var parentKey = DefaultReadonlyEnrichmentStrategy<T>.RowMatchKey(item, enrichment);
            if (parentKey is null) continue;
            if (!index.TryGetValue(parentKey, out var list))
                index[parentKey] = list = [];
            list.Add(item);
        }
        var ids = new HashSet<string>();
        for (var i = 0; i < parents.Count; i++)
        {
            Attach(parents[i], enrichment, index, [], DefaultReadonlyEnrichmentStrategy<T>.At(sources, i));
            var id = DefaultReadonlyEnrichmentStrategy<T>.MatchKey(parents[i], enrichment, DefaultReadonlyEnrichmentStrategy<T>.At(sources, i));
            if (id is not null) ids.Add(id);
        }
        for (var i = parents.Count - 1; i >= 0; i--)
        {
            var parentKey = DefaultReadonlyEnrichmentStrategy<T>.RowMatchKey(parents[i], enrichment);
            if (parentKey is not null && ids.Contains(parentKey))
                parents.RemoveAt(i);
        }
    }

    static void Attach<TRelated>(
        T parent,
        Enrichment enrichment,
        Dictionary<string, List<TRelated>> index,
        HashSet<string> path,
        Row? source)
    {
        var id = DefaultReadonlyEnrichmentStrategy<T>.MatchKey(parent, enrichment, source);
        if (id is null)
        {
            EnrichmentCells.Set(parent, enrichment.DestinationProperty, new List<TRelated>());
            return;
        }
        path.Add(id);
        var children = new List<TRelated>();
        foreach (var child in index.GetValueOrDefault(id) ?? [])
        {
            var childId = DefaultReadonlyEnrichmentStrategy<T>.MatchKey(child, enrichment, null);
            if (childId is null || path.Contains(childId)) continue;
            var copy = EnrichmentCells.CopyExcept(child, enrichment.DestinationProperty);
            Nest(copy, enrichment, index, path);
            children.Add(copy);
        }
        path.Remove(id);
        EnrichmentCells.Set(parent, enrichment.DestinationProperty, children);
    }

    static void Nest<TRelated>(
        TRelated node,
        Enrichment enrichment,
        Dictionary<string, List<TRelated>> index,
        HashSet<string> path)
    {
        var id = DefaultReadonlyEnrichmentStrategy<T>.MatchKey(node, enrichment, null);
        if (id is null)
        {
            EnrichmentCells.Set(node, enrichment.DestinationProperty, new List<TRelated>());
            return;
        }
        path.Add(id);
        var children = new List<TRelated>();
        foreach (var child in index.GetValueOrDefault(id) ?? [])
        {
            var childId = DefaultReadonlyEnrichmentStrategy<T>.MatchKey(child, enrichment, null);
            if (childId is null || path.Contains(childId)) continue;
            var copy = EnrichmentCells.CopyExcept(child, enrichment.DestinationProperty);
            Nest(copy, enrichment, index, path);
            children.Add(copy);
        }
        path.Remove(id);
        EnrichmentCells.Set(node, enrichment.DestinationProperty, children);
    }
}
