namespace Datasources.Mysql;

public sealed class MysqlRepository : IRepository<Dictionary<string, object?>>
{
    public string Table { get; }
    public IConnection Connection { get; }

    public MysqlRepository(MysqlConnection connection, string? table = null)
    {
        Connection = connection;
        Table = table ?? "items";
    }

    public Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null) =>
        throw new ProviderNotImplementedException("mysql");

    public IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null) =>
        throw new ProviderNotImplementedException("mysql");

    public Task<Response> Add(Dictionary<string, object?> obj) =>
        throw new ProviderNotImplementedException("mysql");

    public Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("mysql");

    public Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("mysql");
}
