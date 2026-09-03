using Datasources;

namespace Datasources.Sqlite;

public sealed class SqliteTransactionDriver(SqliteCommandDriver commands)
{
    public async Task Begin(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        await commands.Exec(isolation == IsolationLevel.ReadUncommitted
            ? "PRAGMA read_uncommitted = 1"
            : "PRAGMA read_uncommitted = 0");
        await commands.Exec("BEGIN");
    }

    public Task Commit() => commands.Exec("COMMIT");
    public Task Rollback() => commands.Exec("ROLLBACK");
}
