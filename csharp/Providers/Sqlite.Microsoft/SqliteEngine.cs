using System.Runtime.CompilerServices;

namespace Datasources.Sqlite;

static class SqliteEngine
{
#pragma warning disable CA2255
    [ModuleInitializer]
    internal static void Install() => SqliteSessions.Create = () =>
    {
        var connection = new SqliteConnectionDriver();
        var commands = new SqliteCommandDriver(connection);
        return (connection, commands, new SqliteTransactionDriver(commands));
    };
#pragma warning restore CA2255
}
