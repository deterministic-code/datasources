namespace Datasources.Mysql;

internal sealed class MysqlTransaction : ITransaction
{
    public MysqlTransaction(MysqlConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
    {
        Connection = connection;
        Isolation = isolation;
    }

    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    MysqlConnection Typed => (MysqlConnection)Connection;
    public Task Commit() => Typed.Exec("COMMIT");
    public Task Abort() => Typed.Exec("ROLLBACK");
}
