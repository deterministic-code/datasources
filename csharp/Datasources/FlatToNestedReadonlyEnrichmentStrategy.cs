namespace Datasources;

public sealed class FlatToNestedReadonlyEnrichmentStrategy<T>(
    IReadonlyRepository<Row> source,
    IFlatToNestedMapper mapper,
    IMapper<Dictionary<string, MappedValue>, T> hydrate,
    IStatement? command = null) : IReadonlyEnrichmentStrategy<T>, ITrackedApply where T : new()
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

    public async Task Enrich(IRepositoryTree tree, IList<T> items, IReadOnlyList<Row?>? sources = null)
    {
        _ = tree;
        var rows = new List<Row>();
        if (sources?.Any(row => row is not null) == true)
            rows.AddRange(sources.OfType<Row>());
        else
            await foreach (var row in source.Read(command))
                rows.Add(row);
        items.Clear();
        foreach (var item in mapper.Map(rows).Select(hydrate.Map))
            items.Add(item);
    }

    public Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents) =>
        Task.CompletedTask;

    Task ITrackedApply.ApplyTracked<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents,
        IReadOnlyList<Row?>? sources) =>
        Task.CompletedTask;
}
