namespace Datasources.Mysql;

public sealed class MysqlTransactionDriver(MysqlCommandDriver commands)
{
    public async Task Begin(IsolationLevel isolation = IsolationLevel.Serializable)
    {
        await commands.Exec($"SET TRANSACTION ISOLATION LEVEL {MysqlBind.IsolationSql(isolation)}");
        await commands.Exec("START TRANSACTION");
    }

    public Task Commit() => commands.Exec("COMMIT");
    public Task Rollback() => commands.Exec("ROLLBACK");
}
