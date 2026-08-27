using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteRepository : IRepository<Dictionary<string, object?>>
{
    public string Table { get; }
    public IConnection Connection { get; }
    private SqliteConnection Typed => (SqliteConnection)Connection;

    public SqliteRepository(SqliteConnection connection, string? table = null)
    {
        Connection = connection;
        Table = table ?? "items";
    }

    public async Task<Dictionary<string, object?>?> GetSingle(IQuery? query = null)
    {
        var reader = await new SqliteCommand(Typed).ExecuteReader(SelectSql(query, true));
        return await reader.Next() ? (Dictionary<string, object?>?)ToDict(reader.Current) : null;
    }

    public async IAsyncEnumerable<Dictionary<string, object?>> Get(IQuery? query = null)
    {
        var reader = await new SqliteCommand(Typed).ExecuteReader(SelectSql(query, false));
        while (await reader.Next())
            if (ToDict(reader.Current) is { } record)
                yield return record;
    }

    public Task<Response> Add(Dictionary<string, object?> obj)
    {
        var keys = obj.Keys.ToList();
        var sql = keys.Count == 0
            ? $"INSERT INTO {Table} DEFAULT VALUES"
            : $"INSERT INTO {Table} ({string.Join(", ", keys)}) VALUES ({string.Join(", ", keys.Select(k => $"${k}"))})";
        return new SqliteCommand(Typed).Execute(sql, ToParams(obj));
    }

    public Task<Response> Update(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        var keys = obj.Keys.ToList();
        var assignments = keys.Count == 0 ? "id = id" : string.Join(", ", keys.Select(k => $"{k} = ${k}"));
        var where = byWhere?.GetText() ?? Helpers.IdentityWhereText(obj);
        return new SqliteCommand(Typed).Execute($"UPDATE {Table} SET {assignments} WHERE {where}", ToParams(obj));
    }

    public Task<Response> Delete(Dictionary<string, object?> obj, IWhere? byWhere = null)
    {
        var where = byWhere?.GetText() ?? Helpers.IdentityWhereText(obj);
        return new SqliteCommand(Typed).Execute($"DELETE FROM {Table} WHERE {where}", ToParams(obj));
    }

    private string SelectSql(IQuery? query, bool single) =>
        $"SELECT * FROM {Table}{(query is null ? "" : $" WHERE {query.GetText()}")}{(single ? " LIMIT 1" : "")}";

    private static IReadOnlyList<Parameter> ToParams(Dictionary<string, object?> obj) =>
        obj.Select(p => new Parameter(p.Key, p.Value)).ToList();

    private static Dictionary<string, object?>? ToDict(object? current) =>
        current as Dictionary<string, object?> ??
        (current as IReadOnlyDictionary<string, object?>)?.ToDictionary(k => k.Key, v => v.Value);
}
