namespace Datasources;

public interface IRepository<T> : IReadonlyRepository<T>
{
    Task<Response> Add(T item);
    Task<Response> Update(T item, IWhere? byWhere = null);
    Task<Response> Delete(T item, IWhere? byWhere = null);
}
