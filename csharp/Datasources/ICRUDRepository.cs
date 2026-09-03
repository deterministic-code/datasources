namespace Datasources;

public interface ICRUDRepository<T> : IReadonlyRepository<T>
{
    Task<Response> Create(T item);
    Task<Response> Update(T item, IWhere? byWhere = null);
    Task<Response> Delete(T item, IWhere? byWhere = null);
}
