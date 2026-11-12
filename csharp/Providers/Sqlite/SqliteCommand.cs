using Datasources;
using SqliteCmd = global::Microsoft.Data.Sqlite.SqliteCommand;

namespace Datasources.Sqlite;

internal sealed class SqliteCommand : CommandMiddlewareHost, ICommand
{
    readonly StatementTextRenderer statements;

    public SqliteCommand(SqliteConnection connection, StatementTextRenderer? statements = null)
    {
        Connection = connection;
        this.statements = statements ?? new StatementTextRenderer(t => new SqliteStatementRenderer(t));
    }
    public IConnection Connection { get; }
    SqliteConnection Typed => (SqliteConnection)Connection;

    public Task<IReader<Row>> ExecuteReader(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text.Text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return ((IReader<Row>)new ArrayReader<Row>([]), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var reader = await Open(query, Helpers.ParametersToRecord(paramsList), Schema);
            return await FirstResultReader.Capture(reader);
        });

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(new Native(statements.Render(statement)), parameters);

    public Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement switch
        {
            IReturnStatement or ISelectStatement => FirstCell(statement, parameters),
            InsertStatement { Returning: not null } insert => Inserted(insert, parameters),
            _ => Execute(new Native(statements.Render(statement)), parameters)
        };

    Task<Response> Inserted(InsertStatement insert, IReadOnlyList<Parameter>? parameters) =>
        Run(statements.Render(insert), parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var executed = await RunSql(query, paramsList);
            var response = executed.RowsAffected <= 0
                ? new Response(Value.Null.Instance, executed.RowsAffected, executed.Parameters)
                : new Response(executed.Result, executed.RowsAffected, executed.Parameters);
            return (response, (IReadOnlyList<Row>)[]);
        });

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

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text.Text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return (new Response(Value.Null.Instance, 0, paramsList), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var executed = await RunSql(query, paramsList);
            return (new Response(Value.Null.Instance, executed.RowsAffected, executed.Parameters), (IReadOnlyList<Row>)[]);
        });

    async Task<SqliteExecuteResult> RunSql(string sql, IReadOnlyList<Parameter>? parameters)
    {
        await Typed.Gate.WaitAsync();
        try
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
            using var inserted = Typed.Require().CreateCommand();
            inserted.CommandText = "SELECT last_insert_rowid()";
            return new SqliteExecuteResult(rows, Value.From(await inserted.ExecuteScalarAsync()), parameters);
        }
        finally
        {
            Typed.Gate.Release();
        }
    }

    async Task<IReader<Row>> Open(string sql, Row parameters, Schema? schema = null)
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
                    names ??= Columns(reader);
                    var row = new Row();
                    for (var i = 0; i < reader.FieldCount; i++)
                    {
                        var cell = Value.From(reader.IsDBNull(i) ? null : reader.GetValue(i));
                        row[names[i]] = Converted(schema, names[i], cell);
                    }
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

    static string[] Columns(global::Microsoft.Data.Sqlite.SqliteDataReader reader)
    {
        var names = new string[reader.FieldCount];
        for (var i = 0; i < names.Length; i++)
            names[i] = reader.GetName(i);
        var table = reader.GetSchemaTable();
        if (table is null) return names;
        var duplicated = names.GroupBy(name => name).Where(group => group.Count() > 1).Select(group => group.Key).ToHashSet();
        if (duplicated.Count == 0) return names;
        for (var i = 0; i < names.Length; i++)
        {
            if (!duplicated.Contains(names[i])) continue;
            var baseTable = table.Rows[i]["BaseTableName"]?.ToString();
            var column = table.Rows[i]["BaseColumnName"]?.ToString();
            if (!string.IsNullOrEmpty(baseTable) && !string.IsNullOrEmpty(column))
                names[i] = $"{baseTable}.{column}";
        }
        return names;
    }

    static Value Converted(Schema? schema, string name, Value cell) =>
        schema?.Types.TryGetValue(name, out var type) == true
        && SqliteRowTypeConverters.All.TryGetValue(type, out var converter)
            ? converter.Convert(cell)
            : cell;

    SqliteCmd Bind(string sql, Row parameters)
    {
        var command = Typed.Require().CreateCommand();
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
        var command = Typed.Require().CreateCommand();
        command.CommandText = text;
        for (var index = 0; index < values.Count; index++)
            command.Parameters.AddWithValue($"$p{index}", values[index]);
        return command;
    }

    sealed record SqliteExecuteResult(
        int RowsAffected,
        Value Result,
        IReadOnlyList<Parameter>? Parameters = null);
}
