namespace Datasources;

public sealed class StubRepository<T> : IRepository<T>
{
    private readonly string _provider;
    public StubRepository(string provider, IConnection connection)
    {
        _provider = provider;
        Connection = connection;
    }

    public IConnection Connection { get; }
    public Task<T?> GetSingle(IQuery? query = null) => throw new ProviderNotImplementedException(_provider);
    public Task<T?> GetSingle(IWhere where) => GetSingle((IQuery?)null);
    public IAsyncEnumerable<T> Get(IQuery? query = null) => throw new ProviderNotImplementedException(_provider);
    public IAsyncEnumerable<T> Get(IWhere where) => Get((IQuery?)null);
    public Task<Response> Add(T obj) => throw new ProviderNotImplementedException(_provider);
    public Task<Response> Update(T obj, IWhere? byWhere = null) => throw new ProviderNotImplementedException(_provider);
    public Task<Response> Delete(T obj, IWhere? byWhere = null) => throw new ProviderNotImplementedException(_provider);
}
