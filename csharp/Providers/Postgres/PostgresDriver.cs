namespace Datasources.Postgres;

public sealed class PostgresDriver : IPostgresDriver
{
    readonly PostgresConnectionDriver _connection;
    readonly PostgresCommandDriver _commands;
    readonly PostgresTransactionDriver _transactions;

    public PostgresDriver(string? schema = null)
    {
        _connection = new PostgresConnectionDriver(schema);
        _commands = new PostgresCommandDriver(_connection);
        _transactions = new PostgresTransactionDriver(_commands);
    }

    public Task Open(string url) => _connection.Open(url);
    public Task Close() => _connection.Close();
    public Task<PostgresExecuteResult> Execute(string sql, Row parameters) => _commands.Execute(sql, parameters);
    public Task<PostgresExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters) =>
        _commands.Execute(sql, parameters);
    public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) => _commands.Query(sql, parameters);
    public Task Begin(IsolationLevel isolation = IsolationLevel.Serializable) => _transactions.Begin(isolation);
    public Task Commit() => _transactions.Commit();
    public Task Rollback() => _transactions.Rollback();
}
