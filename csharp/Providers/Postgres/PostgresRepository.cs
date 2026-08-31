namespace Datasources.Postgres;

public sealed class PostgresRepository : IRepository<Dictionary<string, Value>>
{
    public string Table { get; }

    public PostgresRepository(PostgresConnection _, string? table = null) =>
        Table = table ?? "items";

    public Task<Dictionary<string, Value>?> GetSingle(IQuery? query = null) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Dictionary<string, Value>?> GetSingle(IWhere where) => GetSingle((IQuery?)null);

    public IAsyncEnumerable<Dictionary<string, Value>> Get(IQuery? query = null) =>
        throw new ProviderNotImplementedException("postgres");

    public IAsyncEnumerable<Dictionary<string, Value>> Get(IWhere where) => Get((IQuery?)null);

    public Task<Response> Add(Dictionary<string, Value> obj) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Response> Update(Dictionary<string, Value> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Response> Delete(Dictionary<string, Value> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("postgres");
}
