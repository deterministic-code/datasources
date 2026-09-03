namespace Datasources.Mysql;

public sealed class MysqlDriver : IMysqlDriver
{
    readonly MysqlConnectionDriver _connection = new();
    readonly MysqlCommandDriver _commands;
    readonly MysqlTransactionDriver _transactions;

    public MysqlDriver()
    {
        _commands = new MysqlCommandDriver(_connection);
        _transactions = new MysqlTransactionDriver(_commands);
    }

    public Task Open(string url) => _connection.Open(url);
    public Task Close() => _connection.Close();
    public Task<MysqlExecuteResult> Execute(string sql, Row parameters) => _commands.Execute(sql, parameters);
    public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) => _commands.Query(sql, parameters);
    public Task Begin(IsolationLevel isolation = IsolationLevel.Serializable) => _transactions.Begin(isolation);
    public Task Commit() => _transactions.Commit();
    public Task Rollback() => _transactions.Rollback();
}
