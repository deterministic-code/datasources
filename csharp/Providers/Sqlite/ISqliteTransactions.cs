namespace Datasources.Sqlite;

internal interface ISqliteTransactions
{
    Task Begin(IsolationLevel isolation = IsolationLevel.ReadCommitted);
    Task Commit();
    Task Rollback();
}
