namespace Datasources.Postgres;

internal sealed class PostgresTransaction : ITransaction
{
    public PostgresTransaction(PostgresConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    public Task Commit() => ((PostgresConnection)Connection).Driver.Commit();
    public Task Abort() => ((PostgresConnection)Connection).Driver.Rollback();
}
