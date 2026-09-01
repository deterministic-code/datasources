using Datasources;

namespace Datasources.Memory;

public sealed class MemoryRepository : IRepository<Dictionary<string, Value>>
{
    readonly MemoryConnection connection;
    public MemoryRepository(MemoryConnection connection) => this.connection = connection;

    public Task<Dictionary<string, Value>?> GetSingle(IWhere where) =>
        GetSingle(new Query(where.GetText(), where));

    public IAsyncEnumerable<Dictionary<string, Value>> Get(IWhere where) =>
        Get(new Query(where.GetText(), where));

    public async Task<Dictionary<string, Value>?> GetSingle(IQuery? query = null)
    {
        var reader = await new MemoryCommand(connection).ExecuteReader(query?.GetText() ?? "");
        return await reader.Next() ? reader.Current : null;
    }

    public async IAsyncEnumerable<Dictionary<string, Value>> Get(IQuery? query = null)
    {
        var reader = await new MemoryCommand(connection).ExecuteReader(query?.GetText() ?? "");
        while (await reader.Next())
            if (reader.Current is { } record)
                yield return record;
    }

    public Task<Response> Add(Dictionary<string, Value> obj)
    {
        connection.EnsureOpen();
        connection.GetCollection().Add(new Row(obj));
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    public Task<Response> Update(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return new MemoryCommand(connection).Execute("update", ToParams(obj, byWhere));
    }

    public Task<Response> Delete(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return new MemoryCommand(connection).Execute("delete", ToParams(obj, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, Value> obj, IWhere? byWhere) =>
        obj.Select(p => new Parameter(p.Key, p.Value))
            .Concat(byWhere is not null ? [new Parameter("$where", byWhere.GetText())] : [])
            .ToList();
}
