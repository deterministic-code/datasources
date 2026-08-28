namespace Datasources;

public interface IReadonlyRepository<T>
{
    IConnection Connection { get; }
    Task<T?> GetSingle(IQuery? query = null);
    IAsyncEnumerable<T> Get(IQuery? query = null);
}
