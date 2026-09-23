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

    public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql)
            ? Task.FromResult<IReadOnlyList<Row>>([])
            : Read(sql, parameters);

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

    async Task<IReadOnlyList<Row>> Read(string sql, Row parameters)
    {
        using var command = Bind(sql, parameters);
        using var reader = await command.ExecuteReaderAsync();
        var rows = new List<Row>();
        while (await reader.ReadAsync())
        {
            var row = new Row();
            for (var i = 0; i < reader.FieldCount; i++)
                row[reader.GetName(i)] = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
            rows.Add(row);
        }
        return rows;
    }

    SqliteCmd Bind(string sql, Row parameters)
    {
        var command = connection.Require().CreateCommand();
        command.CommandText = sql;
        foreach (var (name, value) in parameters)
        {
            var key = name[0] is '$' or '@' or ':' ? name : $"${name}";
            command.Parameters.AddWithValue(key, value.ToObject() ?? DBNull.Value);
        }
        return command;
    }
}
