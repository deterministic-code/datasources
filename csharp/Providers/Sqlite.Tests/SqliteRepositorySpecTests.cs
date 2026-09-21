using Datasources;
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
            }, CreateTable, SqliteQuote.Quote);
        }
    }

    static string CreateTable(string table, IReadOnlyList<SpecColumn> columns) =>
        new CreateTableStatement(
            SqliteQuote.Quote(table),
            [.. columns.Select(column => new Column(
                SqliteQuote.Quote(column.Name),
                column.Id ? $"{SqliteColumnType.Map(column.Type)} PRIMARY KEY" : SqliteColumnType.Map(column.Type)))]).GetText();
}
