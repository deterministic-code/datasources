using System.Globalization;
using Npgsql;
using NpgsqlTypes;
using DataDirection = System.Data.ParameterDirection;

namespace Datasources.Postgres;

public sealed class PostgresCommandDriver(PostgresConnectionDriver connection)
{
    public Task<PostgresExecuteResult> Execute(string sql, Row parameters) =>
        Execute(sql, parameters.Select(p => new Parameter(p.Key, p.Value)).ToArray());

    public Task<PostgresExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters = null) =>
        string.IsNullOrWhiteSpace(sql)
            ? Task.FromResult(new PostgresExecuteResult(0, Value.Null.Instance, parameters))
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

    async Task<PostgresExecuteResult> Run(string sql, IReadOnlyList<Parameter>? parameters)
    {
        var rewritten = PostgresBind.WithReturning(sql);
        var (text, bound) = PostgresBind.ToPostgres(rewritten, parameters);
        using var command = connection.Require().CreateCommand();
        command.CommandText = text;
        Bind(command, bound);
        if (parameters is not null)
            foreach (var parameter in parameters)
                if (parameter.Direction == ParameterDirection.ReturnValue)
                    command.Parameters.Add(new NpgsqlParameter
                    {
                        Direction = DataDirection.ReturnValue,
                        Value = parameter.Value.ToObject() ?? DBNull.Value
                    });
        if (rewritten.Contains("RETURNING", StringComparison.OrdinalIgnoreCase)
            || rewritten.TrimStart().StartsWith("SELECT", StringComparison.OrdinalIgnoreCase))
        {
            var result = await command.ExecuteScalarAsync();
            return new PostgresExecuteResult(result is null ? 0 : 1, Value.From(result), ReadBack(command, parameters, bound));
        }
        var rows = await command.ExecuteNonQueryAsync();
        return new PostgresExecuteResult(Math.Max(rows, 0), Value.Null.Instance, ReadBack(command, parameters, bound));
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

    static void Bind(NpgsqlCommand command, Parameter[] bound)
    {
        foreach (var parameter in bound)
        {
            var native = Param(Reverse(parameter.Value).ToObject());
            native.Direction = Native(parameter.Direction);
            command.Parameters.Add(native);
        }
    }

    static IReadOnlyList<Parameter>? ReadBack(
        NpgsqlCommand command,
        IReadOnlyList<Parameter>? parameters,
        Parameter[] bound)
    {
        if (parameters is null) return null;
        var values = new Dictionary<string, Value>(StringComparer.Ordinal);
        for (var i = 0; i < bound.Length; i++)
            values[bound[i].Name] = Value.From(command.Parameters[i].Value);
        var ret = parameters.FirstOrDefault(p => p.Direction == ParameterDirection.ReturnValue);
        if (ret is not null && command.Parameters.Count > bound.Length)
            values[ret.Name] = Value.From(command.Parameters[bound.Length].Value);
        return [.. parameters.Select(p => p.Direction == ParameterDirection.Input
            ? p
            : p with { Value = values.TryGetValue(p.Name, out var value) ? value : p.Value })];
    }

    static Value Reverse(Value value) => PostgresBind.Reverse(value);

    static NpgsqlParameter Param(object? value) => value switch
    {
        null => new() { Value = DBNull.Value },
        bool or byte[] => new() { Value = value },
        string text => new() { Value = text, NpgsqlDbType = NpgsqlDbType.Unknown },
        _ => new()
        {
            Value = Convert.ToString(value, CultureInfo.InvariantCulture) ?? "",
            NpgsqlDbType = NpgsqlDbType.Unknown,
        }
    };

    static DataDirection Native(ParameterDirection direction) => direction switch
    {
        ParameterDirection.Output => DataDirection.Output,
        ParameterDirection.InputOutput => DataDirection.InputOutput,
        ParameterDirection.ReturnValue => DataDirection.ReturnValue,
        _ => DataDirection.Input
    };
}
