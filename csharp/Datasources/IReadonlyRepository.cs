namespace Datasources;

public interface IReadonlyRepository<T> : IRepository
{
    Task<T?> ReadSingle(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null);
    Task<T?> ReadSingle(IWhere where);
    IAsyncEnumerable<T> Read(IStatement? query = null, IReadOnlyList<Parameter>? parameters = null);
    IAsyncEnumerable<T> Read(IWhere where);
}
