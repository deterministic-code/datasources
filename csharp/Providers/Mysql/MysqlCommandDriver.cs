using MySqlConnector;

namespace Datasources.Mysql;

public sealed class MysqlCommandDriver(MysqlConnectionDriver connection)
{
    public Task<MysqlExecuteResult> Execute(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql) ? Task.FromResult(new MysqlExecuteResult(0, Value.Null.Instance)) : Run(sql, parameters);

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

    async Task<MysqlExecuteResult> Run(string sql, Row parameters)
    {
        using var command = Bind(sql, parameters);
        var rows = await command.ExecuteNonQueryAsync();
        return new MysqlExecuteResult(rows, Value.From(command.LastInsertedId));
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

    MySqlCommand Bind(string sql, Row parameters)
    {
        var command = connection.Require().CreateCommand();
        command.CommandText = MysqlBind.ToMysql(sql);
        foreach (var (name, value) in parameters)
        {
            var key = name[0] is '@' or ':' or '$' ? name.TrimStart('$', '@', ':') : name;
            command.Parameters.AddWithValue($"@{key}", value.ToObject() ?? DBNull.Value);
        }
        return command;
    }
}
