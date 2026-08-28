using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteTransaction : MiddlewareHost, ITransaction
{
    public SqliteTransaction(SqliteConnection connection, IsolationLevel isolation = IsolationLevel.Serializable)
    {
        Connection = connection;
        Isolation = isolation;
    }
    public IConnection Connection { get; }
    public IsolationLevel Isolation { get; }
    public Task Commit() => Run("commit", () => ((SqliteConnection)Connection).Driver.Commit());
    public Task Abort() => Run("abort", () => ((SqliteConnection)Connection).Driver.Rollback());
}
