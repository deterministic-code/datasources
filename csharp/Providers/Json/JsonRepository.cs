using System.Text.Json.Nodes;
using Datasources;

namespace Datasources.Json;

public sealed class JsonRepository : IRepository<Dictionary<string, Value>>
{
    public JsonRepository(JsonConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private JsonConnection Typed => (JsonConnection)Connection;

    public Task<Dictionary<string, Value>?> GetSingle(IWhere where) =>
        GetSingle(new Query(where.GetText(), where));

    public IAsyncEnumerable<Dictionary<string, Value>> Get(IWhere where) =>
        Get(new Query(where.GetText(), where));

    public async Task<Dictionary<string, Value>?> GetSingle(IQuery? query = null)
    {
        var reader = await new JsonCommand(Typed).ExecuteReader(query?.GetText() ?? "");
        return await reader.Next() ? ToRecord(reader.Current) : null;
    }

    public async IAsyncEnumerable<Dictionary<string, Value>> Get(IQuery? query = null)
    {
        var reader = await new JsonCommand(Typed).ExecuteReader(query?.GetText() ?? "");
        while (await reader.Next())
        {
            var record = ToRecord(reader.Current);
            if (record is not null) yield return record;
        }
    }

    public async Task<Response> Add(Dictionary<string, Value> obj)
    {
        Typed.EnsureOpen();
        Typed.GetCollection().Add(JsonCommand.ToObject(new Row(obj)));
        await Typed.Persist();
        return new Response(Value.Null.Instance, 1);
    }

    public async Task<Response> Update(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return await new JsonCommand(Typed).Execute("update", ToParams(obj, byWhere));
    }

    public async Task<Response> Delete(Dictionary<string, Value> obj, IWhere? byWhere = null)
    {
        Typed.EnsureOpen();
        return await new JsonCommand(Typed).Execute("delete", ToParams(obj, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, Value> obj, IWhere? byWhere) =>
        obj.Select(p => new Parameter(p.Key, p.Value))
            .Concat(byWhere is not null ? [new Parameter("$where", byWhere.GetText())] : [])
            .ToList();

    private static Dictionary<string, Value>? ToRecord(object? current) => current switch
    {
        Row row => row,
        Dictionary<string, Value> row => row,
        JsonObject obj => obj.ToDictionary(k => k.Key, v => JsonCommand.JsonValueToValue(v.Value)),
        _ => null
    };
}
