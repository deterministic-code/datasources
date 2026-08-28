namespace Datasources;

public interface IRepository<T> : IReadonlyRepository<T>
{
    Task<Response> Add(T obj);
    Task<Response> Update(T obj, IWhere? byWhere = null);
    Task<Response> Delete(T obj, IWhere? byWhere = null);
}
