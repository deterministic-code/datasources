using Datasources;

namespace Datasources.Sqlite;

internal sealed class SqliteDriver : ISqliteDriver
{
    readonly SqliteConnectionDriver _connection = new();
    readonly SqliteCommandDriver _commands;
    readonly SqliteTransactionDriver _transactions;

    public SqliteDriver()
    {
        _commands = new SqliteCommandDriver(_connection);
        _transactions = new SqliteTransactionDriver(_commands);
    }

    public Task Open(string filename) => _connection.Open(filename);
    public Task Close() => _connection.Close();
    public Task<SqliteExecuteResult> Execute(string sql, Row parameters) => _commands.Execute(sql, parameters);
    public Task<SqliteExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters) =>
        _commands.Execute(sql, parameters);
    public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) => _commands.Query(sql, parameters);
    public Task Begin(IsolationLevel isolation = IsolationLevel.Serializable) => _transactions.Begin(isolation);
    public Task Commit() => _transactions.Commit();
    public Task Rollback() => _transactions.Rollback();
}
