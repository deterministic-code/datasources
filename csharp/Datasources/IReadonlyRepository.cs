namespace Datasources;

public interface IReadonlyRepository<T>
{
    Task<T?> GetSingle(IQuery? query = null);
    Task<T?> GetSingle(IWhere where);
    IAsyncEnumerable<T> Get(IQuery? query = null);
    IAsyncEnumerable<T> Get(IWhere where);
}
