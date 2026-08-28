using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteTransaction : ITransaction
{
    public SqliteTransaction(SqliteConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    public Task Commit() => ((SqliteConnection)Connection).Driver.Commit();
    public Task Abort() => ((SqliteConnection)Connection).Driver.Rollback();
}
