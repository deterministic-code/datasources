namespace Datasources;

public interface IBulkWriteStrategy<T> where T : new()
{
    Task<Response> Create(IReadOnlyList<T> items, BulkCRUDRepository<T> repo);
    Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo);
    Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, BulkCRUDRepository<T> repo);
}
