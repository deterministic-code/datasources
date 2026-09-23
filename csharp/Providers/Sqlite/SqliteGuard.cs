using System.Reflection;
using Datasources;

namespace Datasources.Sqlite;

internal static class SqliteGuard
{
    internal static ISqliteDriver? Default { get; set; }

    internal static ISqliteDriver? Engine()
    {
        if (Default is not null) return Default;
        try { Assembly.Load("Datasources.Sqlite.Microsoft"); }
        catch (Exception ex) when (ex is FileNotFoundException or FileLoadException) { }
        return Default;
    }

    public static ISqliteDriver Require(ISqliteDriver? driver) =>
        driver ?? throw new DatasourceException("SQLite driver is required");
}
