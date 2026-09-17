using System.Globalization;
using Npgsql;
using NpgsqlTypes;

namespace Datasources.Postgres;

public sealed class PostgresCommandDriver(PostgresConnectionDriver connection)
{
    public Task<PostgresExecuteResult> Execute(string sql, Row parameters) =>
        string.IsNullOrWhiteSpace(sql) ? Task.FromResult(new PostgresExecuteResult(0, Value.Null.Instance)) : Run(sql, parameters);

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

    async Task<PostgresExecuteResult> Run(string sql, Row parameters)
    {
        var rewritten = PostgresBind.WithReturning(sql);
        var (text, values) = PostgresBind.ToPostgres(rewritten, parameters);
        using var command = connection.Require().CreateCommand();
        command.CommandText = text;
        Bind(command, values);
        if (rewritten.Contains("RETURNING", StringComparison.OrdinalIgnoreCase))
        {
            var result = await command.ExecuteScalarAsync();
            return new PostgresExecuteResult(result is null ? 0 : 1, Value.From(result));
        }
        var rows = await command.ExecuteNonQueryAsync();
        return new PostgresExecuteResult(Math.Max(rows, 0), Value.Null.Instance);
    }

    async Task<IReadOnlyList<Row>> Read(string sql, Row parameters)
    {
        var (text, values) = PostgresBind.ToPostgres(sql, parameters);
        using var command = connection.Require().CreateCommand();
        command.CommandText = text;
        Bind(command, values);
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

    static void Bind(NpgsqlCommand command, object?[] values)
    {
        for (var i = 0; i < values.Length; i++)
            command.Parameters.Add(Param(values[i]));
    }

    static NpgsqlParameter Param(object? value) => value switch
    {
        null => new() { Value = DBNull.Value },
        bool or byte[] => new() { Value = value },
        DateTime date => new() { Value = new Value.DateTime(date).IsoDateTime(), NpgsqlDbType = NpgsqlDbType.Unknown },
        string text => new() { Value = text, NpgsqlDbType = NpgsqlDbType.Unknown },
        _ => new()
        {
            Value = Convert.ToString(value, CultureInfo.InvariantCulture) ?? "",
            NpgsqlDbType = NpgsqlDbType.Unknown,
        }
    };
}
