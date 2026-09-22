using Datasources.Mysql;
using Datasources.Postgres;
using Datasources.Sqlite;

namespace UserRolesSimpleCli;

public static class ColumnTypes
{
    public static Func<string, string> Map(string provider) => provider switch
    {
        "sqlite" => SqliteColumnType.Map,
        "mysql" => MysqlColumnType.Map,
        "postgres" => PostgresColumnType.Map,
        _ => throw new InvalidOperationException($"unknown provider {provider}")
    };
}
