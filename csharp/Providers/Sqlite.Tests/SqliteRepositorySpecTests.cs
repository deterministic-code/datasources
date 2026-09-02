using Datasources.Sqlite;
using Datasources.SqlSpecs;

namespace Datasources.Sqlite.Tests;

public class SqliteRepositorySpecTests
{
    [Fact]
    public async Task UserCrud()
    {
        foreach (var path in RepositorySpec.Files())
        {
            await RepositorySpec.Run(path, () =>
            {
                var conn = new SqliteConnection(new SqliteConnectionOptions
                {
                    Filename = ":memory:",
                    Driver = new SqliteDriver(),
                });
                return new RepositorySpec.Session(conn, new SqliteCommand(conn));
            }, (table, columns) => SqliteStatementBuilder.CreateTable(table,
                [.. columns.Select(column => new SqliteColumn(column.Name, column.Type, column.Id))]).GetText(),
                SqliteStatementBuilder.Quote);
        }
    }
}
