using Datasources;

namespace Datasources.Memory;

public sealed class MemoryRepository : IRepository<Dictionary<string, object?>>
{
    public MemoryRepository(MemoryConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MemoryConnection Typed => (MemoryConnection)Connection;

    public async Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null)
    {
        var reader = await new MemoryCommand(Typed).ExecuteReader(query?.GetText());
        return await reader.Next() ? reader.Current as Dictionary<string, object?> : null;
    }

    public async IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null)
    {
        var reader = await new MemoryCommand(Typed).ExecuteReader(query?.GetText());
        while (await reader.Next())
            if (reader.Current is Dictionary<string, object?> record)
                yield return record;
    }

    public Task<Response> Add(Dictionary<string, object?> obj)
    {
        Typed.EnsureOpen();
        Typed.GetCollection().Add(new Dictionary<string, object?>(obj));
        return Task.FromResult(new Response(obj, 1));
    }

    public Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return new MemoryCommand(Typed).Execute("update", ToParams(obj, byWhere));
    }

    public Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return new MemoryCommand(Typed).Execute("delete", ToParams(obj, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, object?> obj, IWhere? byWhere) =>
        obj.Select(p => new Parameter(p.Key, p.Value))
            .Concat(byWhere is not null ? [new Parameter("$where", byWhere.GetText())] : [])
            .ToList();
}
