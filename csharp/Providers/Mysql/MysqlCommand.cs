using System.Data;
using MySqlConnector;
using DataDirection = System.Data.ParameterDirection;

namespace Datasources.Mysql;

internal sealed class MysqlCommand : CommandMiddlewareHost, ICommand
{
    public MysqlCommand(MysqlConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    MysqlConnection Typed => (MysqlConnection)Connection;

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

    async Task<MysqlExecuteResult> RunSql(string sql, IReadOnlyList<Parameter>? parameters)
    {
        using var command = Bind(sql, parameters);
        if (sql.TrimStart().StartsWith("SELECT", StringComparison.OrdinalIgnoreCase))
        {
            var scalar = await command.ExecuteScalarAsync();
            return new MysqlExecuteResult(scalar is null ? 0 : 1, Value.From(scalar), ReadBack(command, parameters));
        }
        var rows = await command.ExecuteNonQueryAsync();
        return new MysqlExecuteResult(rows, Value.From(command.LastInsertedId), ReadBack(command, parameters));
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

    MySqlCommand Bind(string sql, IReadOnlyList<Parameter>? parameters)
    {
        var command = Typed.Require().CreateCommand();
        var list = parameters ?? [];
        if (sql.TrimStart().StartsWith("CALL", StringComparison.OrdinalIgnoreCase)
            && list.Any(p => p.Direction != Datasources.ParameterDirection.Input))
        {
            command.CommandType = CommandType.StoredProcedure;
            command.CommandText = ProcedureName(sql);
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

    static IReadOnlyList<Parameter>? ReadBack(MySqlCommand command, IReadOnlyList<Parameter>? parameters) =>
        parameters is null ? null
        : [.. parameters.Select(p => p.Direction == Datasources.ParameterDirection.Input
            ? p
            : p with { Value = Value.From(command.Parameters[$"@{p.Name.TrimStart('$', '@', ':')}"].Value) })];

    static string ProcedureName(string sql)
    {
        var body = sql.Trim();
        if (body.StartsWith("CALL", StringComparison.OrdinalIgnoreCase))
            body = body[4..].Trim();
        var open = body.IndexOf('(');
        return open < 0 ? body : body[..open].Trim().Trim('`');
    }

    static DataDirection Native(Datasources.ParameterDirection direction) => direction switch
    {
        Datasources.ParameterDirection.Output => DataDirection.Output,
        Datasources.ParameterDirection.InputOutput => DataDirection.InputOutput,
        Datasources.ParameterDirection.ReturnValue => DataDirection.ReturnValue,
        _ => DataDirection.Input
    };
}
