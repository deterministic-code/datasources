namespace Datasources;

public interface IWriteEnrichmentStrategy<T>
{
    Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode);
}
