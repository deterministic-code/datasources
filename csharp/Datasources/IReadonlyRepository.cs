namespace Datasources;

public interface IReadonlyRepository<T> : IRepository
{
    Task<T?> GetSingle(IStatement? query = null);
    IAsyncEnumerable<T> Get(IStatement? query = null);
}
