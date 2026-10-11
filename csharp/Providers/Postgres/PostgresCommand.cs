using System.Data;
using System.Globalization;
using Npgsql;
using NpgsqlTypes;
using DataDirection = System.Data.ParameterDirection;

namespace Datasources.Postgres;

internal sealed class PostgresCommand : CommandMiddlewareHost, ICommand
{
    readonly StatementTextRenderer statements;

    public PostgresCommand(PostgresConnection connection, StatementTextRenderer? statements = null)
    {
        Connection = connection;
        this.statements = statements ?? new StatementTextRenderer(t => new PostgresStatementRenderer(t));
    }
    public IConnection Connection { get; }
    PostgresConnection Typed => (PostgresConnection)Connection;

    public Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text.Text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return ((IReader<Row>)new ArrayReader<Row>([]), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var reader = await Open(query, Helpers.ParametersToRecord(paramsList));
            return await FirstResultReader.Capture(reader);
        });

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text.Text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return (new Response(Value.Null.Instance, 0, paramsList), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var result = await RunSql(query, paramsList);
            return (new Response(result.Result, result.RowsAffected, result.Parameters), (IReadOnlyList<Row>)[]);
        });

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(new Native(statements.Render(statement)), parameters);

    public Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement switch
        {
            IReturnStatement or ISelectStatement => FirstCell(statement, parameters),
            ExecuteStoredProcedureStatement { Name: { } name } when HasOutput(parameters) =>
                ExecuteProcedure(name, parameters),
            _ => Execute(new Native(statements.Render(statement)), parameters)
        };

    async Task<Response> FirstCell(IStatement statement, IReadOnlyList<Parameter>? parameters)
    {
        var reader = await ExecuteReader(statement, parameters);
        try
        {
            if (!await reader.Next())
                return new Response(Value.Null.Instance, 0, parameters);
            var row = reader.Current ?? throw new DatasourceException("Reader current must be a row");
            var cell = row.Count == 0 ? Value.Null.Instance : row.First().Value;
            return new Response(cell, 1, parameters);
        }
        finally
        {
            if (reader is IAsyncDisposable disposable)
                await disposable.DisposeAsync();
        }
    }

    Task<Response> ExecuteProcedure(string name, IReadOnlyList<Parameter>? parameters) =>
        Run(name, parameters, async (_, list) =>
        {
            Typed.EnsureOpen();
            using var command = Typed.Require().CreateCommand();
            command.CommandType = CommandType.StoredProcedure;
            command.CommandText = name;
            foreach (var parameter in list ?? [])
            {
                var key = parameter.Name.TrimStart('$', '@', ':');
                var native = Param(PostgresRowTypeConverters.Reverse(parameter.Value).ToObject());
                native.ParameterName = key;
                native.Direction = Native(parameter.Direction);
                native.NpgsqlDbType = NpgsqlDbType.Jsonb;
                command.Parameters.Add(native);
            }
            var rows = Math.Max(await command.ExecuteNonQueryAsync(), 0);
            return (new Response(Value.Null.Instance, rows, AppliedNamed(command, list)), (IReadOnlyList<Row>)[]);
        });

    async Task<PostgresExecuteResult> RunSql(string sql, IReadOnlyList<Parameter>? parameters)
    {
        var rows = 0;
        IReadOnlyList<Parameter>? applied = parameters;
        foreach (var part in SplitStatements.Split(sql))
        {
            var names = SplitStatements.Parameters(part).ToHashSet();
            var used = parameters is null
                ? null
                : parameters.Where(parameter => names.Contains(parameter.Name.TrimStart('$', '@', ':'))).ToArray();
            var rewritten = PostgresBind.ToPostgres(part, used);
            using var command = Typed.Require().CreateCommand();
            command.CommandText = rewritten.Sql;
            Bind(command, rewritten.Bound);
            if (used is not null)
                foreach (var parameter in used)
                    if (parameter.Direction == ParameterDirection.ReturnValue)
                        command.Parameters.Add(new NpgsqlParameter
                        {
                            Direction = DataDirection.ReturnValue,
                            Value = parameter.Value.ToObject() ?? DBNull.Value
                        });
            rows += Math.Max(await command.ExecuteNonQueryAsync(), 0);
            applied = Applied(command, parameters, rewritten.Bound);
        }
        return new PostgresExecuteResult(rows, Value.Null.Instance, applied);
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

    static void Bind(NpgsqlCommand command, object?[] values)
    {
        for (var i = 0; i < values.Length; i++)
            command.Parameters.Add(Param(values[i]));
    }

    static void Bind(NpgsqlCommand command, Parameter[] bound)
    {
        foreach (var parameter in bound)
        {
            var native = Param(PostgresRowTypeConverters.Reverse(parameter.Value).ToObject());
            native.Direction = Native(parameter.Direction);
            command.Parameters.Add(native);
        }
    }

    static bool HasOutput(IReadOnlyList<Parameter>? parameters) =>
        parameters?.Any(parameter => parameter.Direction != ParameterDirection.Input) == true;

    static IReadOnlyList<Parameter>? AppliedNamed(NpgsqlCommand command, IReadOnlyList<Parameter>? parameters) =>
        parameters is null ? null
        : [.. parameters.Select(parameter =>
        {
            if (parameter.Direction == ParameterDirection.Input) return parameter;
            var key = parameter.Name.TrimStart('$', '@', ':');
            return command.Parameters.IndexOf(key) is var at and >= 0
                ? parameter with { Value = Value.From(command.Parameters[at].Value) }
                : parameter;
        })];

    static IReadOnlyList<Parameter>? Applied(
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
        string text when BinaryConverter.IsBase64(text) => new() { Value = System.Convert.FromBase64String(text.Trim()) },
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
