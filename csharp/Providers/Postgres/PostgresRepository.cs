namespace Datasources.Postgres;

public sealed class PostgresRepository : IRepository<Dictionary<string, object?>>
{
    public string Table { get; }
    public IConnection Connection { get; }

    public PostgresRepository(PostgresConnection connection, string? table = null)
    {
        Connection = connection;
        Table = table ?? "items";
    }

    public Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Dictionary<string, object?>?> GetSingle(IWhere where) => GetSingle((IQuery?)null);

    public IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null) =>
        throw new ProviderNotImplementedException("postgres");

    public IAsyncEnumerable<Dictionary<string, object?>> Get(IWhere where) => Get((IQuery?)null);

    public Task<Response> Add(Dictionary<string, object?> obj) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("postgres");
}
