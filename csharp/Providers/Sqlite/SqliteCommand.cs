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
        Execute(new SqliteStatementFactory().Transform(statement).GetText(), parameters);

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
