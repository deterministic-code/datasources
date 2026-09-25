using Datasources;
using SqliteCmd = Microsoft.Data.Sqlite.SqliteCommand;

namespace Datasources.Sqlite;

internal sealed class SqliteCommandDriver(SqliteConnectionDriver connection) : ISqliteCommands
{
    public Task<SqliteExecuteResult> Execute(string sql, Row parameters) =>
        Execute(sql, parameters.Select(p => new Parameter(p.Key, p.Value)).ToArray());

    public Task<SqliteExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(sql)
            ? Task.FromResult(new SqliteExecuteResult(0, Value.Null.Instance, parameters))
            : Run(sql, parameters);

    public IAsyncEnumerable<Row> Query(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql) ? EmptyRows() : Read(sql, parameters);

    public async Task<IReader<Row>> Open(string sql, Row parameters)
    {
        var command = Bind(sql, parameters);
        try
        {
            var reader = await command.ExecuteReaderAsync();
            string[]? names = null;
            return new StreamingReader(
                async () =>
                {
                    if (!await reader.ReadAsync()) return null;
                    names ??= ColumnNames(reader);
                    var row = new Row();
                    for (var i = 0; i < reader.FieldCount; i++)
                        row[names[i]] = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
                    return row;
                },
                async () =>
                {
                    names = null;
                    return await reader.NextResultAsync();
                },
                async () =>
                {
                    await reader.DisposeAsync();
                    await command.DisposeAsync();
                });
        }
        catch
        {
            await command.DisposeAsync();
            throw;
        }
    }

    internal async Task Exec(string sql)
    {
        using var command = connection.Require().CreateCommand();
        command.CommandText = sql;
        await command.ExecuteNonQueryAsync();
    }

    async Task<SqliteExecuteResult> Run(string sql, IReadOnlyList<Parameter>? parameters)
    {
        using var command = Bind(sql, Helpers.ParametersToRecord(parameters));
        if (sql.TrimStart().StartsWith("SELECT", StringComparison.OrdinalIgnoreCase))
        {
            var scalar = await command.ExecuteScalarAsync();
            return new SqliteExecuteResult(scalar is null ? 0 : 1, Value.From(scalar), parameters);
        }
        var rows = await command.ExecuteNonQueryAsync();
        using var id = connection.Require().CreateCommand();
        id.CommandText = "SELECT last_insert_rowid()";
        return new SqliteExecuteResult(rows, Value.From(await id.ExecuteScalarAsync()), parameters);
    }

    static async IAsyncEnumerable<Row> EmptyRows()
    {
        yield break;
    }

    async IAsyncEnumerable<Row> Read(string sql, Row parameters)
    {
        using var command = Bind(sql, parameters);
        using var reader = await command.ExecuteReaderAsync();
        var names = ColumnNames(reader);
        while (await reader.ReadAsync())
        {
            var row = new Row();
            for (var i = 0; i < reader.FieldCount; i++)
                row[names[i]] = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
            yield return row;
        }
    }

    static string[] ColumnNames(Microsoft.Data.Sqlite.SqliteDataReader reader)
    {
        var names = new string[reader.FieldCount];
        for (var i = 0; i < names.Length; i++)
            names[i] = reader.GetName(i);
        var schema = reader.GetSchemaTable();
        var duplicated = names.GroupBy(name => name).Where(group => group.Count() > 1).Select(group => group.Key).ToHashSet();
        if (schema is null || duplicated.Count == 0) return names;
        for (var i = 0; i < names.Length; i++)
        {
            if (!duplicated.Contains(names[i])) continue;
            var table = schema.Rows[i]["BaseTableName"]?.ToString();
            var column = schema.Rows[i]["BaseColumnName"]?.ToString();
            if (!string.IsNullOrEmpty(table) && !string.IsNullOrEmpty(column))
                names[i] = $"{table}.{column}";
        }
        return names;
    }

    SqliteCmd Bind(string sql, Row parameters)
    {
        var command = connection.Require().CreateCommand();
        command.CommandText = sql;
        foreach (var (name, value) in parameters)
        {
            var key = name[0] is '$' or '@' or ':' ? name : $"${name}";
            command.Parameters.AddWithValue(key, SqliteRowTypeConverters.Reverse(value).ToObject() ?? DBNull.Value);
        }
        return command;
    }
}
