using Datasources;
using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace Datasources.Sqlite.Tests;

public class SqliteRepositorySpecTests
{
    [Fact]
    public async Task UserCrud()
    {
        foreach (var path in RepositoryTestsSpec.Files())
        {
            await RepositoryTestsSpec.Run(path, () =>
            {
                var conn = new SqliteConnection(new SqliteConnectionOptions
                {
                    Filename = ":memory:",
                });
                return new RepositoryTestsSpec.Session(conn, new SqliteCommand(conn), SqliteRowTypeConverters.All);
            }, Quote);
        }
    }

    static string Quote(string id) => $"\"{id.Replace("\"", "\"\"")}\"";
}
