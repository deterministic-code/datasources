namespace Datasources.Mysql;

public sealed class MysqlCommand : CommandMiddlewareHost, ICommand
{
    public MysqlCommand(MysqlConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private MysqlConnection Typed => (MysqlConnection)Connection;

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
            var result = await Typed.Driver.Execute(query, paramsList);
            return (Response.Of(result.Result, result.RowsAffected, result.Parameters), (IReadOnlyList<Row>)[]);
        });
}
