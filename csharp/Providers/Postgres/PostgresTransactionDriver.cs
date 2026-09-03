namespace Datasources.Postgres;

public sealed class PostgresTransactionDriver(PostgresCommandDriver commands)
{
    public Task Begin(IsolationLevel isolation = IsolationLevel.Serializable) =>
        commands.Exec($"BEGIN ISOLATION LEVEL {PostgresBind.IsolationSql(isolation)}");

    public Task Commit() => commands.Exec("COMMIT");
    public Task Rollback() => commands.Exec("ROLLBACK");
}
