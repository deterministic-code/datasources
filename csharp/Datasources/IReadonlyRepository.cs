namespace Datasources;

public interface IReadonlyRepository<T>
{
    IConnection Connection { get; }
    Task<T?> GetSingle(IQuery? query = null);
    Task<T?> GetSingle(IWhere where);
    IAsyncEnumerable<T> Get(IQuery? query = null);
    IAsyncEnumerable<T> Get(IWhere where);
}
