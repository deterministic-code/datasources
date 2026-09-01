using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteCommand : CommandMiddlewareHost, ICommand
{
    public SqliteCommand(SqliteConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private SqliteConnection Typed => (SqliteConnection)Connection;

    public Task<IReader<Row>> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var rows = await Typed.Driver.Query(query, Helpers.ParametersToRecord(paramsList));
            return ((IReader<Row>)new ArrayReader<Row>(rows), rows);
        });

    public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var result = await Typed.Driver.Execute(query, Helpers.ParametersToRecord(paramsList));
            return (new Response(result.Result, result.RowsAffected), (IReadOnlyList<Row>)[]);
        });
}
