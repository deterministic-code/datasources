namespace Datasources.Mysql;

public sealed class MysqlRepository : ReadonlyRepository<Dictionary<string, Value>>, IRepository<Dictionary<string, Value>>
{
    public string Table { get; }

    public MysqlRepository(MysqlConnection _, string? table = null) =>
        Table = table ?? "items";

    public override IAsyncEnumerable<Dictionary<string, Value>> Get(IStatement? query = null) =>
        throw new ProviderNotImplementedException("mysql");

    public Task<Response> Add(Dictionary<string, Value> item) =>
        throw new ProviderNotImplementedException("mysql");

    public Task<Response> Update(Dictionary<string, Value> item, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("mysql");

    public Task<Response> Delete(Dictionary<string, Value> item, IWhere? byWhere = null) =>
        throw new ProviderNotImplementedException("mysql");
}
