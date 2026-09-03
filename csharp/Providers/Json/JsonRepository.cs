using Datasources;

namespace Datasources.Json;

public sealed class JsonRepository : ReadonlyRepository<Dictionary<string, Value>>, ICRUDRepository<Dictionary<string, Value>>
{
    readonly JsonConnection connection;
    public JsonRepository(JsonConnection connection) => this.connection = connection;

    public override async IAsyncEnumerable<Dictionary<string, Value>> Get(IStatement? query = null)
    {
        var reader = await new JsonCommand(connection).ExecuteReader(query?.GetText() ?? "");
        while (await reader.Next())
        {
            if (reader.Current is { } record) yield return record;
        }
    }

    public async Task<Response> Create(Dictionary<string, Value> item)
    {
        connection.EnsureOpen();
        connection.GetCollection().Add(JsonCommand.ToObject(new Row(item)));
        await connection.Persist();
        return new Response(Value.Null.Instance, 1);
    }

    public async Task<Response> Update(Dictionary<string, Value> item, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return await new JsonCommand(connection).Execute("update", ToParams(item, byWhere));
    }

    public async Task<Response> Delete(Dictionary<string, Value> item, IWhere? byWhere = null)
    {
        connection.EnsureOpen();
        return await new JsonCommand(connection).Execute("delete", ToParams(item, byWhere));
    }

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, Value> item, IWhere? byWhere) =>
        item.Select(p => new Parameter(p.Key, p.Value))
            .Concat(byWhere is not null ? [new Parameter("$where", byWhere.GetText())] : [])
            .ToList();

}
