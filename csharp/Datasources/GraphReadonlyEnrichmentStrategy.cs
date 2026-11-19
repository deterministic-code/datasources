namespace Datasources;

public sealed class GraphReadonlyEnrichmentStrategy<T, TLink> : IReadonlyEnrichmentStrategy<T>
    where T : new()
    where TLink : new()
{
    readonly IReadonlyRepository<TLink> link;

    public GraphReadonlyEnrichmentStrategy(IReadonlyRepository<TLink> link) => this.link = link;

    public Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null) =>
        Fill(via, enrichment, parents, sources);

    public Task ApplyLinked<TRelated, TJunction>(
        IReadonlyRepository<TRelated> via,
        IReadonlyRepository<TJunction> junction,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources = null) =>
        Apply(via, enrichment, parents, sources);

    async Task Fill<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IList<T> parents,
        IReadOnlyList<Row?>? sources)
    {
        var association = enrichment.Association ?? throw new DatasourceException("Missing association");
        var nodes = new Dictionary<string, TRelated>();
        await foreach (var item in via.Read())
        {
            var id = DefaultReadonlyEnrichmentStrategy<T>.MatchKey(item, enrichment, null);
            if (id is not null)
                nodes[id] = item;
        }
        var outgoing = new Dictionary<string, List<string>>();
        await foreach (var edge in link.Read())
        {
            var from = EnrichmentCells.Get(edge, association.Parent);
            var to = EnrichmentCells.Get(edge, association.Related);
            if (from is null || to is null) continue;
            var fromKey = DefaultReadonlyEnrichmentStrategy<T>.KeyOf(from);
            var toKey = DefaultReadonlyEnrichmentStrategy<T>.KeyOf(to);
            if (!outgoing.TryGetValue(fromKey, out var list))
                outgoing[fromKey] = list = [];
            list.Add(toKey);
        }
        var filled = new HashSet<string>();
        for (var i = 0; i < parents.Count; i++)
        {
            var id = DefaultReadonlyEnrichmentStrategy<T>.MatchKey(parents[i], enrichment, DefaultReadonlyEnrichmentStrategy<T>.At(sources, i));
            if (id is null)
            {
                EnrichmentCells.Set(parents[i], enrichment.DestinationProperty, new List<TRelated>());
                continue;
            }
            filled.Add(id);
            EnrichmentCells.Set(parents[i], enrichment.DestinationProperty, Walk(id, enrichment, nodes, outgoing, [id], filled));
        }
    }

    static List<TRelated> Walk<TRelated>(
        string from,
        Enrichment enrichment,
        Dictionary<string, TRelated> nodes,
        Dictionary<string, List<string>> outgoing,
        HashSet<string> path,
        HashSet<string> filled)
    {
        var edges = new List<TRelated>();
        foreach (var to in outgoing.GetValueOrDefault(from) ?? [])
        {
            if (path.Contains(to) || !nodes.TryGetValue(to, out var node)) continue;
            var first = filled.Add(to);
            var copy = EnrichmentCells.CopyExcept(node, enrichment.DestinationProperty);
            if (first)
            {
                path.Add(to);
                EnrichmentCells.Set(copy, enrichment.DestinationProperty, Walk(to, enrichment, nodes, outgoing, path, filled));
                path.Remove(to);
            }
            else
                EnrichmentCells.Set(copy, enrichment.DestinationProperty, new List<TRelated>());
            edges.Add(copy);
        }
        return edges;
    }
}
