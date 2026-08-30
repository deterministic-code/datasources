using Datasources;

namespace Datasources.Memory;

public sealed class MemoryRepository : IRepository<Dictionary<string, Value>>
{
    public MemoryRepository(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public Task<Dictionary<string, Value>?> GetSingle(IWhere where) =>
        GetSingle(new Query(where.GetText(), where));

    public IAsyncEnumerable<Dictionary<string, Value>> Get(IWhere where) =>
        Get(new Query(where.GetText(), where));

    public async Task<Dictionary<string, Value>?> GetSingle(IQuery? query = null)
    {
        var reader = await new MemoryCommand(Typed).ExecuteReader(query?.GetText());
        return await reader.Next() ? reader.Current as Dictionary<string, Value> : null;
    }

    public async IAsyncEnumerable<Dictionary<string, Value>> Get(IQuery? query = null)
    {
        var reader = await new MemoryCommand(Typed).ExecuteReader(query?.GetText());
        while (await reader.Next())
            if (reader.Current is Dictionary<string, Value> record)
                yield return record;
    }

    public Task<Response> Add(Dictionary<string, Value> obj)
    {
        Typed.EnsureOpen();
        Typed.GetCollection().Add(new Row(obj));
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    public Task<Response> Update(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return new MemoryCommand(Typed).Execute("update", ToParams(obj, byWhere));
    }

    public Task<Response> Delete(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return new MemoryCommand(Typed).Execute("delete", ToParams(obj, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, Value> obj, IWhere? byWhere) =>
        obj.Select(p => new Parameter(p.Key, p.Value))
            .Concat(byWhere is not null ? [new Parameter("$where", byWhere.GetText())] : [])
            .ToList();
}
