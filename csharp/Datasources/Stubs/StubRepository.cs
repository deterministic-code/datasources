namespace Datasources;

public sealed class StubRepository<T> : ReadonlyRepository<T>, ICRUDRepository<T> where T : new()
{
    readonly string provider;

    public StubRepository(string provider, IConnection _) => this.provider = provider;

    public IReadOnlyList<string> Ids { get; } = [];

    public override IAsyncEnumerable<T> Read(
        IStatement? query = null,
        IReadOnlyList<Parameter>? parameters = null,
        ReadonlyRepositoryMappingOptions<T>? options = null) =>
        throw new ProviderNotImplementedException(provider);
    public Task<T> Create(T item, IWriteEnrichmentStrategy<T>? writeEnrichment = null) =>
        throw new ProviderNotImplementedException(provider);
    public Task<Response> Update(T item, IWhere? byWhere = null, IWriteEnrichmentStrategy<T>? writeEnrichment = null) =>
        throw new ProviderNotImplementedException(provider);
    public Task<Response> Delete(T item, IWhere? byWhere = null, IWriteEnrichmentStrategy<T>? writeEnrichment = null) =>
        throw new ProviderNotImplementedException(provider);
    public Task<T?> ReadSingleById(Value id) => throw new ProviderNotImplementedException(provider);
    public Task<T?> ReadSingleById(Row ids) => throw new ProviderNotImplementedException(provider);
}
