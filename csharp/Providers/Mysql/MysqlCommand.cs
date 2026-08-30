namespace Datasources.Mysql;

public sealed class MysqlCommand : CommandMiddlewareHost, ICommand
{
    public MysqlCommand(MysqlConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MysqlConnection Typed => (MysqlConnection)Connection;

    public Task<IReader> ExecuteReader(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var rows = await Typed.Driver.Query(query, Helpers.ParametersToRecord(paramsList));
            return ((IReader)new ArrayReader(rows.Cast<object?>().ToList()), rows);
        });

    public Task<Response> Execute(string text, IReadOnlyList<Parameter>? parameters = null) =>
        Run(text, parameters, async (query, paramsList) =>
        {
            Typed.EnsureOpen();
            var result = await Typed.Driver.Execute(query, Helpers.ParametersToRecord(paramsList));
            return (new Response(result.Result, result.RowsAffected), (IReadOnlyList<Row>)[]);
        });
}
