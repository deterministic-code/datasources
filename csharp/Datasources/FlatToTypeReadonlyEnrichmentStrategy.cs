namespace Datasources;

public sealed class FlatToTypeReadonlyEnrichmentStrategy<T>(
    IReadonlyRepository<Row> source,
    IFlatToTypeMapper mapper,
    IMapper<Dictionary<string, MappedValue>, T> hydrate,
    IStatement? command = null) : IReadonlyEnrichmentStrategy<T>
{
    public async IAsyncEnumerable<T> Get(
        IReadonlyRepository<T> inner,
        IRepositoryTree tree,
        IStatement? query)
    {
        _ = inner;
        _ = tree;
        var rows = new List<Row>();
        await foreach (var row in source.Get(command ?? query))
            rows.Add(row);
        foreach (var item in mapper.Collapse(rows).Select(hydrate.Map))
            yield return item;
    }

    public Task Enrich(IRepositoryTree tree, IList<T> items) => Task.CompletedTask;

    public Task Apply<TRelated>(
        IReadonlyRepository<TRelated> via,
        Enrichment enrichment,
        IReadOnlyList<object> parents) =>
        Task.CompletedTask;
}
