namespace Datasources.Mysql;

public sealed class MysqlTransaction : ITransaction
{
    public MysqlTransaction(MysqlConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    public Task Commit() => ((MysqlConnection)Connection).Driver.Commit();
    public Task Abort() => ((MysqlConnection)Connection).Driver.Rollback();
}
