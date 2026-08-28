using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteRepository : IRepository<Dictionary<string, object?>>
{
    public string Table { get; }
    public IConnection Connection { get; }

    public SqliteRepository(SqliteConnection connection, string? table = null)
    {
        Connection = connection;
        Table = table ?? "items";
    }

    public Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null) =>
        throw new ProviderNotImplementedException("sqlite");

    public IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null) =>
        throw new ProviderNotImplementedException("sqlite");

    public Task<Response> Add(Dictionary<string, object?> obj) =>
        throw new ProviderNotImplementedException("sqlite");

    public Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("sqlite");

    public Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("sqlite");
}
