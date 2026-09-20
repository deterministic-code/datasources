namespace Datasources;

public interface IReadonlyRepository<T> : IRepository
{
    Task<T?> ReadSingle(IStatement? query = null);
    IAsyncEnumerable<T> Read(IStatement? query = null);
}
