namespace Datasources;

public interface IRepository<T>
{
    IConnection Connection { get; }
    Task<T?> GetSingle(IQuery? query = null);
    IAsyncEnumerable<T> Get(IQuery? query = null);
    Task<Response> Add(T obj);
    Task<Response> Update(T obj, IWhere? byWhere = null);
    Task<Response> Delete(T obj, IWhere? byWhere = null);
}
