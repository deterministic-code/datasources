namespace Datasources;

public sealed class StubRepository<T> : IRepository<T>
{
    readonly string provider;

    public StubRepository(string provider, IConnection _) => this.provider = provider;

    public Task<T?> GetSingle(IQuery? query = null) => throw new ProviderNotImplementedException(provider);
    public Task<T?> GetSingle(IWhere where) => GetSingle((IQuery?)null);
    public IAsyncEnumerable<T> Get(IQuery? query = null) => throw new ProviderNotImplementedException(provider);
    public IAsyncEnumerable<T> Get(IWhere where) => Get((IQuery?)null);
    public Task<Response> Add(T item) => throw new ProviderNotImplementedException(provider);
    public Task<Response> Update(T item, IWhere? byWhere = null) => throw new ProviderNotImplementedException(provider);
    public Task<Response> Delete(T item, IWhere? byWhere = null) => throw new ProviderNotImplementedException(provider);
}
