using System.Data;
using MySqlConnector;
using DataDirection = System.Data.ParameterDirection;

namespace Datasources.Mysql;

internal sealed class MysqlCommand : CommandMiddlewareHost, ICommand
{
    readonly StatementTextRenderer statements;

    public MysqlCommand(MysqlConnection connection, StatementTextRenderer? statements = null)
    {
        Connection = connection;
        this.statements = statements ?? new StatementTextRenderer(t => new MysqlStatementRenderer(t));
    }
    public IConnection Connection { get; }
    MysqlConnection Typed => (MysqlConnection)Connection;

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
            using var command = Bind(name, list, procedure: true);
            var rows = await command.ExecuteNonQueryAsync();
            return (new Response(Value.Null.Instance, Math.Max(rows, 0), Applied(command, list)), (IReadOnlyList<Row>)[]);
        });

    async Task<MysqlExecuteResult> RunSql(string sql, IReadOnlyList<Parameter>? parameters)
    {
        using var command = Bind(sql, parameters);
        var rows = await command.ExecuteNonQueryAsync();
        return new MysqlExecuteResult(Math.Max(rows, 0), Value.Null.Instance, Applied(command, parameters));
    }

    async Task<StreamingReader> Open(string sql, Row parameters)
    {
        var command = Bind(sql, parameters);
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

    MySqlCommand Bind(string sql, IReadOnlyList<Parameter>? parameters, bool procedure = false)
    {
        var command = Typed.Require().CreateCommand();
        var list = parameters ?? [];
        if (procedure)
        {
            command.CommandType = CommandType.StoredProcedure;
            command.CommandText = sql;
        }
        else
            command.CommandText = MysqlBind.ToMysql(sql);
        foreach (var parameter in list)
        {
            var key = parameter.Name.TrimStart('$', '@', ':');
            var native = command.Parameters.AddWithValue($"@{key}", MysqlRowTypeConverters.Reverse(parameter.Value).ToObject() ?? DBNull.Value);
            native.Direction = Native(parameter.Direction);
        }
        return command;
    }

    MySqlCommand Bind(string sql, Row parameters)
    {
        var command = Typed.Require().CreateCommand();
        command.CommandText = MysqlBind.ToMysql(sql);
        foreach (var (name, value) in parameters)
        {
            var key = name[0] is '@' or ':' or '$' ? name.TrimStart('$', '@', ':') : name;
            command.Parameters.AddWithValue($"@{key}", MysqlRowTypeConverters.Reverse(value).ToObject() ?? DBNull.Value);
        }
        return command;
    }

    static bool HasOutput(IReadOnlyList<Parameter>? parameters) =>
        parameters?.Any(parameter => parameter.Direction != Datasources.ParameterDirection.Input) == true;

    static IReadOnlyList<Parameter>? Applied(MySqlCommand command, IReadOnlyList<Parameter>? parameters) =>
        parameters is null ? null
        : [.. parameters.Select(p => p.Direction == Datasources.ParameterDirection.Input
            ? p
            : p with { Value = Value.From(command.Parameters[$"@{p.Name.TrimStart('$', '@', ':')}"].Value) })];

    static DataDirection Native(Datasources.ParameterDirection direction) => direction switch
    {
        Datasources.ParameterDirection.Output => DataDirection.Output,
        Datasources.ParameterDirection.InputOutput => DataDirection.InputOutput,
        Datasources.ParameterDirection.ReturnValue => DataDirection.ReturnValue,
        _ => DataDirection.Input
    };
}
