namespace Datasources.Sqlite;

internal interface ISqliteTransactions
{
    Task Begin(IsolationLevel isolation = IsolationLevel.Serializable);
    Task Commit();
    Task Rollback();
}
