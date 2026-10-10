namespace Datasources;

public interface IWriteEnrichmentStrategy<T>
{
    bool UseTransaction => false;

    Task Persist(IRepositoryTree tree, IList<T> items, WriteMode mode);

    Task PersistBulk(IRepositoryTree tree, IList<T> items, WriteMode mode) => Persist(tree, items, mode);
}
