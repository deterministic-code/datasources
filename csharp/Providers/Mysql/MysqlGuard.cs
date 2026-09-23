namespace Datasources.Mysql;

internal static class MysqlGuard
{
    public static IMysqlDriver Require(IMysqlDriver? driver) =>
        driver ?? throw new DatasourceException("MySQL driver is required");
}
