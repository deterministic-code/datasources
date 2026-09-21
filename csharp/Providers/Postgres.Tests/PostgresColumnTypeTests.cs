using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresColumnTypeTests
{
    static readonly PostgresColumn[] UserColumns =
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
        Assert.Equal("INTEGER", PostgresColumnType.Map("int"));
        var error = Assert.Throws<DatasourceException>(() => PostgresColumnType.Map("xml"));
        Assert.Contains("Unknown postgres type \"xml\"", error.Message);
    }

    [Fact]
    public void CreatesUserTable() =>
        Assert.Equal(
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" UUID, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" DOUBLE PRECISION, \"created\" TIMESTAMPTZ, \"version\" BYTEA)",
            new CreateTableStatement(
                PostgresQuote.Quote("user"),
                [.. UserColumns.Select(column => new Column(
                    PostgresQuote.Quote(column.Name),
                    column.Id ? $"{PostgresColumnType.Map(column.Type)} PRIMARY KEY" : PostgresColumnType.Map(column.Type)))]).GetText());
}
