namespace Datasources.Postgres;

public sealed class PostgresCommand : CommandMiddlewareHost, ICommand
{
    public PostgresCommand(PostgresConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private PostgresConnection Typed => (PostgresConnection)Connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var rows = await Typed.Driver.Query(query, Helpers.ParametersToRecord(paramsList));
            return ((IReader)new ArrayReader(rows.Cast<object?>().ToList()), rows);
        });

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var result = await Typed.Driver.Execute(query, Helpers.ParametersToRecord(paramsList));
            return (new Response(result.Result, result.RowsAffected), (IReadOnlyList<IReadOnlyDictionary<string, object?>>)[]);
        });
}
