using System.Runtime.CompilerServices;

namespace Datasources.Sqlite;

static class SqliteEngine
{
#pragma warning disable CA2255
    [ModuleInitializer]
    internal static void Install() => SqliteGuard.Default = new SqliteDriver();
#pragma warning restore CA2255
}
