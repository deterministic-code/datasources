using Datasources;
using Datasources.Builders;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteStatementBuilderTests
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
    public void QuotesAndMapsTypes()
    {
        Assert.Equal("\"user\"", SqliteStatementBuilder.Quote("user"));
        Assert.Equal("\"a\"\"b\"", SqliteStatementBuilder.Quote("a\"b"));
        Assert.Equal("INTEGER", SqliteStatementBuilder.ColumnType("int"));
        var error = Assert.Throws<DatasourceException>(() => SqliteStatementBuilder.ColumnType("xml"));
        Assert.Contains("Unknown sqlite type \"xml\"", error.Message);
    }

    [Fact]
    public void CreatesUserTable() =>
        Assert.Equal(
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" TEXT, \"username\" TEXT, \"is_enabled\" INTEGER, \"balance\" NUMERIC, \"location\" REAL, \"created\" TEXT, \"version\" BLOB)",
            SqliteStatementBuilder.CreateTable("user", UserColumns).GetText());

    [Fact]
    public void QuotesTablesInDml()
    {
        var where = WhereBuilder.Equals("id", 1);
        Parameter[] fields = [new("id", 1), new("username", "alice")];
        Assert.Equal(
            "SELECT \"id\", \"username\" FROM \"user\" WHERE id = $id",
            SqliteStatementBuilder.CreateQuery("user", ["id", "username"], where).GetText());
        Assert.Equal(
            "INSERT INTO \"user\" (id, username) VALUES ($id, $username)",
            SqliteStatementBuilder.CreateInsert("user", fields).GetText());
        Assert.Equal(
            "UPDATE \"user\" SET id = $id, username = $username WHERE id = $id",
            SqliteStatementBuilder.CreateUpdate("user", fields, where).GetText());
        Assert.Equal(
            "DELETE FROM \"user\" WHERE id = $id",
            SqliteStatementBuilder.CreateDelete("user", where).GetText());
    }
}
