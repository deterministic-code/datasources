namespace Datasources;

public interface IReadonlyRepository<T> : IRepository
{
    Task<T?> ReadSingle(IStatement? query = null);
    Task<T?> ReadSingle(IWhere where);
    IAsyncEnumerable<T> Read(IStatement? query = null);
    IAsyncEnumerable<T> Read(IWhere where);
}
