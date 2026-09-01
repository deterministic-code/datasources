using Datasources;

namespace Datasources.Json;

public sealed class JsonRepository : IRepository<Dictionary<string, Value>>
{
    readonly JsonConnection connection;
    public JsonRepository(JsonConnection connection) => this.connection = connection;

    public Task<Dictionary<string, Value>?> GetSingle(IWhere where) =>
        GetSingle(new Query(where.GetText(), where));

    public IAsyncEnumerable<Dictionary<string, Value>> Get(IWhere where) =>
        Get(new Query(where.GetText(), where));

    public async Task<Dictionary<string, Value>?> GetSingle(IQuery? query = null)
    {
        var reader = await new JsonCommand(connection).ExecuteReader(query?.GetText() ?? "");
        return await reader.Next() ? reader.Current : null;
    }

    public async IAsyncEnumerable<Dictionary<string, Value>> Get(IQuery? query = null)
    {
        var reader = await new JsonCommand(connection).ExecuteReader(query?.GetText() ?? "");
        while (await reader.Next())
        {
            if (reader.Current is { } record) yield return record;
        }
    }

    public async Task<Response> Add(Dictionary<string, Value> obj)
    {
        connection.EnsureOpen();
        connection.GetCollection().Add(JsonCommand.ToObject(new Row(obj)));
        await connection.Persist();
        return new Response(Value.Null.Instance, 1);
    }

    public async Task<Response> Update(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return await new JsonCommand(connection).Execute("update", ToParams(obj, byWhere));
    }

    public async Task<Response> Delete(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return await new JsonCommand(connection).Execute("delete", ToParams(obj, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, Value> obj, IWhere? byWhere) =>
        obj.Select(p => new Parameter(p.Key, p.Value))
            .Concat(byWhere is not null ? [new Parameter("$where", byWhere.GetText())] : [])
            .ToList();

}
