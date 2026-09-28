namespace Datasources;

public interface IBulkWrite<T>
{
    ICommand Command { get; }
    Task<Value> CreateOne(T item);
    Task<Response> UpdateOne(T item, IWhere? byWhere);
    Task<Response> DeleteOne(T item, IWhere? byWhere);
    IReadOnlyList<BulkCommand> Inserts(IReadOnlyList<T> items);
    IReadOnlyList<BulkCommand> Updates(IReadOnlyList<T> items, IWhere? byWhere);
    IReadOnlyList<BulkCommand> Deletes(IReadOnlyList<T> items, IWhere? byWhere);
    IReadOnlyList<Row> Rows(IReadOnlyList<T> items);
}
