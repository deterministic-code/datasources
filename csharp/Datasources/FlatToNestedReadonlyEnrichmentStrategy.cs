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
        _ = inner;
        _ = tree;
        var rows = new List<Row>();
        await foreach (var row in source.Read(command ?? query))
            rows.Add(row);
        foreach (var item in mapper.Map(rows).Select(hydrate.Map))
            yield return item;
    }

    public Task Enrich(IRepositoryTree tree, IList<T> items, IReadOnlyList<Row?>? sources = null) =>
        Task.CompletedTask;

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
