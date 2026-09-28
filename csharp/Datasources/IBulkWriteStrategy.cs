namespace Datasources;

public interface IBulkWriteStrategy<T>
{
    Task<Response> Create(IReadOnlyList<T> items, IBulkWrite<T> write);
    Task<Response> Update(IReadOnlyList<T> items, IWhere? byWhere, IBulkWrite<T> write);
    Task<Response> Delete(IReadOnlyList<T> items, IWhere? byWhere, IBulkWrite<T> write);
}
