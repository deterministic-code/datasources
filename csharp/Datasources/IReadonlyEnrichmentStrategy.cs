namespace Datasources;

public interface IReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentBinder
{
    IAsyncEnumerable<T> Read(
        IReadonlyRepository<T> inner,
        IRepositoryTree tree,
        IStatement? query);

    Task Enrich(IRepositoryTree tree, IList<T> items, IReadOnlyList<Row?>? sources = null);
}
