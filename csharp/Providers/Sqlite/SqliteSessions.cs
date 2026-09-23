using System.Reflection;

namespace Datasources.Sqlite;

internal static class SqliteSessions
{
    internal static Func<(ISqliteConnectionSession Connection, ISqliteCommands Commands, ISqliteTransactions Transactions)>? Create { get; set; }

    internal static (ISqliteConnectionSession Connection, ISqliteCommands Commands, ISqliteTransactions Transactions) New()
    {
        if (Create is null)
        {
            try { Assembly.Load("Datasources.Sqlite.Microsoft"); }
            catch (Exception ex) when (ex is FileNotFoundException or FileLoadException) { }
        }
        return Create?.Invoke() ?? throw new DatasourceException("SQLite driver is required");
    }
}
