namespace Datasources;

public sealed class StubRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T>
{
    readonly string provider;

    public StubRepository(string provider, IConnection _) => this.provider = provider;

    public override IAsyncEnumerable<T> Get(IStatement? query = null) =>
        throw new ProviderNotImplementedException(provider);
    public Task<Response> Create(T item) => throw new ProviderNotImplementedException(provider);
    public Task<Response> Update(T item, IWhere? byWhere = null) => throw new ProviderNotImplementedException(provider);
    public Task<Response> Delete(T item, IWhere? byWhere = null) => throw new ProviderNotImplementedException(provider);
}
