using Datasources;

namespace Datasources.Memory;

public sealed class MemoryRepository : ReadonlyRepository<Dictionary<string, Value>>, IRepository<Dictionary<string, Value>>
{
    readonly MemoryConnection connection;
    public MemoryRepository(MemoryConnection connection) => this.connection = connection;

    public override async IAsyncEnumerable<Dictionary<string, Value>> Get(IStatement? query = null)
    {
        var reader = await new MemoryCommand(connection).ExecuteReader(query?.GetText() ?? "");
        while (await reader.Next())
            if (reader.Current is { } record)
                yield return record;
    }

    public Task<Response> Add(Dictionary<string, Value> item)
    {
        connection.EnsureOpen();
        connection.GetCollection().Add(new Row(item));
        return Task.FromResult(new Response(Value.Null.Instance, 1));
    }

    public Task<Response> Update(Dictionary<string, Value> item, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return new MemoryCommand(connection).Execute("update", ToParams(item, byWhere));
    }

    public Task<Response> Delete(Dictionary<string, Value> item, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return new MemoryCommand(connection).Execute("delete", ToParams(item, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, Value> item, IWhere? byWhere) =>
        item.Select(p => new Parameter(p.Key, p.Value))
            .Concat(byWhere is not null ? [new Parameter("$where", byWhere.GetText())] : [])
            .ToList();
}
