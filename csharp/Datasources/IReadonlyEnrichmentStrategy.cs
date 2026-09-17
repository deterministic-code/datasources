namespace Datasources;

public interface IReadonlyEnrichmentStrategy<T> : IReadonlyEnrichmentBinder
{
    IAsyncEnumerable<T> Get(
        IReadonlyRepository<T> inner,
        IRepositoryTree tree,
        IStatement? query);

    Task Enrich(IRepositoryTree tree, IList<T> items);
}
