using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteCommand : MiddlewareHost, ICommand
{
    public SqliteCommand(SqliteConnection connection) => Connection = connection;
    public IConnection Connection { get; }
    private SqliteConnection Typed => (SqliteConnection)Connection;

    public Task<IReader> ExecuteReader(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("executeReader", async () =>
        {
            Typed.EnsureOpen();
            var rows = await Typed.Driver.Query(text ?? "", Helpers.ParametersToRecord(parameters));
            return (IReader)new ArrayReader(rows.Cast<object?>().ToList());
        }, text, parameters);

    public Task<Response> Execute(string? text = null, IReadOnlyList<Parameter>? parameters = null) =>
        Run("execute", async () =>
        {
            Typed.EnsureOpen();
            var result = await Typed.Driver.Execute(text ?? "", Helpers.ParametersToRecord(parameters));
            return new Response(result.Result, result.RowsAffected);
        }, text, parameters);
}
