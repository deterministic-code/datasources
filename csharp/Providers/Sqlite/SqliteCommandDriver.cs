using Datasources;
using SqliteCmd = Microsoft.Data.Sqlite.SqliteCommand;

namespace Datasources.Sqlite;

internal sealed class SqliteCommandDriver(SqliteConnectionDriver connection) : ISqliteCommands
{
    public Task<SqliteExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(sql)
            ? Task.FromResult(new SqliteExecuteResult(0, Value.Null.Instance, parameters))
            : Run(sql, parameters);

    public async Task<IReader<Row>> Open(string sql, Row parameters)
    {
        var command = Bind(sql, parameters);
        try
        {
            var reader = await command.ExecuteReaderAsync();
            string[]? names = null;
            bool[]? booleans = null;
            return new StreamingReader(
                async () =>
                {
                    if (!await reader.ReadAsync()) return null;
                    if (names is null)
                        (names, booleans) = Columns(reader);
                    var row = new Row();
                    for (var i = 0; i < reader.FieldCount; i++)
                    {
                        var cell = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
                        row[names[i]] = booleans![i] ? SqliteRowTypeConverters.All["boolean"].Convert(cell) : cell;
                    }
                    return row;
                },
                async () =>
                {
                    names = null;
                    booleans = null;
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
        var rows = 0;
        foreach (var part in SplitStatements.Split(sql))
        {
            var names = SplitStatements.Parameters(part).ToHashSet();
            var used = new Row();
            foreach (var (name, value) in Helpers.ParametersToRecord(parameters))
                if (names.Contains(name.TrimStart('$', '@', ':')))
                    used[name] = value;
            using var command = BindPositional(part, used);
            rows += Math.Max(await command.ExecuteNonQueryAsync(), 0);
        }
        return new SqliteExecuteResult(rows, Value.Null.Instance, parameters);
    }

    static (string[] Names, bool[] Booleans) Columns(Microsoft.Data.Sqlite.SqliteDataReader reader)
    {
        var names = new string[reader.FieldCount];
        var booleans = new bool[reader.FieldCount];
        for (var i = 0; i < names.Length; i++)
            names[i] = reader.GetName(i);
        var schema = reader.GetSchemaTable();
        if (schema is null) return (names, booleans);
        for (var i = 0; i < names.Length; i++)
            booleans[i] = string.Equals(schema.Rows[i]["DataTypeName"]?.ToString(), "BOOLEAN", StringComparison.OrdinalIgnoreCase);
        var duplicated = names.GroupBy(name => name).Where(group => group.Count() > 1).Select(group => group.Key).ToHashSet();
        if (duplicated.Count == 0) return (names, booleans);
        for (var i = 0; i < names.Length; i++)
        {
            if (!duplicated.Contains(names[i])) continue;
            var table = schema.Rows[i]["BaseTableName"]?.ToString();
            var column = schema.Rows[i]["BaseColumnName"]?.ToString();
            if (!string.IsNullOrEmpty(table) && !string.IsNullOrEmpty(column))
                names[i] = $"{table}.{column}";
        }
        return (names, booleans);
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

    SqliteCmd BindPositional(string sql, Row parameters)
    {
        var values = new List<object?>();
        var text = System.Text.RegularExpressions.Regex.Replace(sql, @"\$([A-Za-z_][A-Za-z0-9_]*)", match =>
        {
            var name = match.Groups[1].Value;
            var index = values.Count;
            values.Add(parameters.TryGetValue(name, out var value)
                ? SqliteRowTypeConverters.Reverse(value).ToObject() ?? DBNull.Value
                : DBNull.Value);
            return $"$p{index}";
        });
        var command = connection.Require().CreateCommand();
        command.CommandText = text;
        for (var index = 0; index < values.Count; index++)
            command.Parameters.AddWithValue($"$p{index}", values[index]);
        return command;
    }
}
