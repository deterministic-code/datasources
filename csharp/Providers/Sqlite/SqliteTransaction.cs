namespace Datasources.Sqlite;

internal sealed class SqliteTransaction : ITransaction
{
    public SqliteTransaction(SqliteConnection connection, IsolationLevel isolation = IsolationLevel.ReadCommitted)
    {
        Connection = connection;
        Isolation = isolation;
    }

    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    SqliteConnection Typed => (SqliteConnection)Connection;
    public Task Commit() => Typed.Transactions.Commit();
    public Task Abort() => Typed.Transactions.Rollback();
}
