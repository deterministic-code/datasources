namespace Datasources;

public interface ICRUDRepository<T> : IReadonlyRepository<T>
{
    IReadOnlyList<string> Ids { get; }
    Task<Response> Create(T item);
    Task<Response> Update(T item, IWhere? byWhere = null);
    Task<Response> Delete(T item, IWhere? byWhere = null);
    Task<T?> ReadSingleById(Value id);
    Task<T?> ReadSingleById(Row ids);
}
