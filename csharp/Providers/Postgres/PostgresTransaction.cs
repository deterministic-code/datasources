namespace Datasources.Postgres;

internal sealed class PostgresTransaction : ITransaction
{
    public PostgresTransaction(PostgresConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted)
    {
        Connection = connection;
        Isolation = isolation;
    }

    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    PostgresConnection Typed => (PostgresConnection)Connection;
    public Task Commit() => Typed.Exec("COMMIT");
    public Task Abort() => Typed.Exec("ROLLBACK");
}
