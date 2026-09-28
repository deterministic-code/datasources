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
            }, Quote, Map);
        }
    }

    static string Quote(string id) => $"\"{id.Replace("\"", "\"\"")}\"";

    static string Map(string canonical) => canonical switch
    {
        "int" => "INTEGER",
        "uuid" => "TEXT",
        "string" => "TEXT",
        "bool" => "INTEGER",
        "decimal" => "NUMERIC",
        "float" => "REAL",
        "datetime" => "TEXT",
        "binary" => "BLOB",
        _ => throw new InvalidOperationException($"Unknown sqlite type \"{canonical}\"")
    };
}
