namespace Datasources.Mysql;

internal static class IsolationSql
{
    public static string Of(IsolationLevel isolation) => isolation switch
    {
        IsolationLevel.ReadUncommitted => "READ UNCOMMITTED",
        IsolationLevel.ReadCommitted => "READ COMMITTED",
        _ => "REPEATABLE READ"
    };
}
