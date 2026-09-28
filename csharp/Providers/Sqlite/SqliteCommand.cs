namespace Datasources.Sqlite;

internal sealed class SqliteCommand : CommandMiddlewareHost, ICommand
{
    public SqliteCommand(SqliteConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    SqliteConnection Typed => (SqliteConnection)Connection;

    public Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return ((IReader<Row>)new ArrayReader<Row>([]), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var reader = await Typed.Commands.Open(query, Helpers.ParametersToRecord(paramsList));
            return (reader, (IReadOnlyList<Row>)[]);
        });

    public Task<IReader<Row>> ExecuteReader(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        ExecuteReader(new SqliteStatementFactory().Transform(statement).GetText(), parameters);

    public Task<Response> Execute(IStatement statement, IReadOnlyList<Parameter>? parameters = null) =>
        statement switch
        {
            IReturnStatement or ISelectStatement => FirstCell(statement, parameters),
            _ => Execute(new SqliteStatementFactory().Transform(statement).GetText(), parameters)
        };

    async Task<Response> FirstCell(IStatement statement, IReadOnlyList<Parameter>? parameters)
    {
        var reader = await ExecuteReader(statement, parameters);
        try
        {
            if (!await reader.Next())
                return Response.Of(Value.Null.Instance, 0, parameters);
            var row = reader.Current ?? throw new DatasourceException("Reader current must be a row");
            var cell = row.Count == 0 ? Value.Null.Instance : row.First().Value;
            return Response.Of(cell, 1, parameters);
        }
        finally
        {
            if (reader is IAsyncDisposable disposable)
                await disposable.DisposeAsync();
        }
    }

    public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            if (string.IsNullOrWhiteSpace(query))
                return (Response.Of(Value.Null.Instance, 0, paramsList), (IReadOnlyList<Row>)[]);
            Typed.EnsureOpen();
            var result = await Typed.Commands.Execute(query, paramsList);
            return (Response.Of(result.Result, result.RowsAffected, result.Parameters), (IReadOnlyList<Row>)[]);
        });
}
