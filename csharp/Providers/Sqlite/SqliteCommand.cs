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
            var reader = await Typed.Commands.Open(query, Helpers.ParametersToRecord(paramsList));
            return await FirstResultReader.Capture(reader);
        });

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(new Native(statements.Render(statement)), parameters);

    public Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement switch
        {
            IReturnStatement or ISelectStatement => FirstCell(statement, parameters),
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

    public Task<Response> Execute(Native text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text.Text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return (new Response(Value.Null.Instance, 0, paramsList), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var result = await Typed.Commands.Execute(query, paramsList);
            return (new Response(result.Result, result.RowsAffected, result.Parameters), (IReadOnlyList<Row>)[]);
        });
}
