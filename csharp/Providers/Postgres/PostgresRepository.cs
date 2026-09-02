namespace Datasources.Postgres;

public sealed class PostgresRepository : ReadonlyRepository<Dictionary<string, Value>>, IRepository<Dictionary<string, Value>>
{
    public string Table { get; }

    public PostgresRepository(PostgresConnection _, string? table = null) =>
        Table = table ?? "items";

    public override IAsyncEnumerable<Dictionary<string, Value>> Get(IStatement? query = null) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Response> Add(Dictionary<string, Value> item) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Response> Update(Dictionary<string, Value> item, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("postgres");

    public Task<Response> Delete(Dictionary<string, Value> item, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("postgres");
}
