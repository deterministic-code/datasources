using System.Globalization;
using Npgsql;
using NpgsqlTypes;
using DataDirection = System.Data.ParameterDirection;

namespace Datasources.Postgres;

internal sealed class PostgresCommand : CommandMiddlewareHost, ICommand
{
    public PostgresCommand(PostgresConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    PostgresConnection Typed => (PostgresConnection)Connection;

    public Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return ((IReader<Row>)new ArrayReader<Row>([]), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var reader = await Open(query, Helpers.ParametersToRecord(paramsList));
            return (reader, (IReadOnlyList<Row>)[]);
        });

    public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return (Response.Of(Value.Null.Instance, 0, paramsList), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var result = await RunSql(query, paramsList);
            return (Response.Of(result.Result, result.RowsAffected, result.Parameters), (IReadOnlyList<Row>)[]);
        });

    async Task<PostgresExecuteResult> RunSql(string sql, IReadOnlyList<Parameter>? parameters)
    {
        var rewritten = PostgresBind.WithReturning(sql);
        var (text, bound) = PostgresBind.ToPostgres(rewritten, parameters);
        using var command = Typed.Require().CreateCommand();
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

    async Task<StreamingReader> Open(string sql, Row parameters)
    {
        var (text, values) = PostgresBind.ToPostgres(sql, parameters);
        var command = Typed.Require().CreateCommand();
        command.CommandText = text;
        Bind(command, values);
        try
        {
            var reader = await command.ExecuteReaderAsync();
            return new StreamingReader(
                async () =>
                {
                    if (!await reader.ReadAsync()) return null;
                    var row = new Row();
                    for (var i = 0; i < reader.FieldCount; i++)
                        row[reader.GetName(i)] = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
                    return row;
                },
                () => reader.NextResultAsync(),
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

    async IAsyncEnumerable<Row> Read(string sql, Row parameters)
    {
        var (text, values) = PostgresBind.ToPostgres(sql, parameters);
        using var command = Typed.Require().CreateCommand();
        command.CommandText = text;
        Bind(command, values);
        using var reader = await command.ExecuteReaderAsync();
        while (await reader.ReadAsync())
        {
            var row = new Row();
            for (var i = 0; i < reader.FieldCount; i++)
                row[reader.GetName(i)] = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
            yield return row;
        }
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
            var native = Param(PostgresBind.Reverse(parameter.Value).ToObject());
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
