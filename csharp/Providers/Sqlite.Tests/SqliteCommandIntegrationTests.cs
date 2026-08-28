using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace Datasources.Sqlite.Tests;

public class SqliteCommandIntegrationTests
{
    [Fact]
    public async Task SharedSpecs()
    {
        var dialect = SqlSpec.Dialect.Load("sqlite");
        foreach (var path in Directory.GetFiles(SqlSpec.SpecsDir(), "*.yaml"))
            await SqlSpec.Run(path, dialect, target =>
            {
                var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = target, Driver = new SqliteDriver() });
                return new SqlSpec.Session(conn, new SqliteCommand(conn));
            }, () => new SqliteCommand(new SqliteConnection(new SqliteConnectionOptions { Driver = new SqliteDriver() })));
    }
}
