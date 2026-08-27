using Datasources;

namespace Datasources.Sqlite;

public static class SqliteGuard
{
    public static ISqliteDriver Require(ISqliteDriver? driver) =>
        driver ?? throw new DatasourceException("SQLite driver is required");
}
