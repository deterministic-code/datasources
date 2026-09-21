using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteColumnTypeTests
{
    static readonly SqliteColumn[] UserColumns =
    [
        new("id", "int", true),
        new("uuid", "uuid"),
        new("username", "string"),
        new("is_enabled", "bool"),
        new("balance", "decimal"),
        new("location", "float"),
        new("created", "datetime"),
        new("version", "binary"),
    ];

    [Fact]
    public void MapsKnownAndUnknown()
    {
        Assert.Equal("INTEGER", SqliteColumnType.Map("int"));
        var error = Assert.Throws<DatasourceException>(() => SqliteColumnType.Map("xml"));
        Assert.Contains("Unknown sqlite type \"xml\"", error.Message);
    }

    [Fact]
    public void CreatesUserTable() =>
        Assert.Equal(
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" TEXT, \"username\" TEXT, \"is_enabled\" INTEGER, \"balance\" NUMERIC, \"location\" REAL, \"created\" TEXT, \"version\" BLOB)",
            new CreateTableStatement(
                SqliteQuote.Quote("user"),
                [.. UserColumns.Select(column => new Column(
                    SqliteQuote.Quote(column.Name),
                    column.Id ? $"{SqliteColumnType.Map(column.Type)} PRIMARY KEY" : SqliteColumnType.Map(column.Type)))]).GetText());
}
