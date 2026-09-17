using Datasources;
using Datasources.Builders;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresStatementBuilderTests
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
    public void QuotesAndMapsTypes()
    {
        Assert.Equal("\"user\"", PostgresStatementBuilder.Quote("user"));
        Assert.Equal("\"a\"\"b\"", PostgresStatementBuilder.Quote("a\"b"));
        Assert.Equal("INTEGER", PostgresStatementBuilder.ColumnType("int"));
        var error = Assert.Throws<DatasourceException>(() => PostgresStatementBuilder.ColumnType("xml"));
        Assert.Contains("Unknown postgres type \"xml\"", error.Message);
    }

    [Fact]
    public void CreatesUserTable() =>
        Assert.Equal(
            "CREATE TABLE \"user\" (\"id\" INTEGER PRIMARY KEY, \"uuid\" UUID, \"username\" TEXT, \"is_enabled\" BOOLEAN, \"balance\" NUMERIC, \"location\" DOUBLE PRECISION, \"created\" TIMESTAMPTZ, \"version\" BYTEA)",
            PostgresStatementBuilder.CreateTable("user", UserColumns).GetText());

    [Fact]
    public void QuotesTablesInDml()
    {
        var where = WhereBuilder.Equals("id", 1);
        Parameter[] fields = [new("id", 1), new("username", "alice")];
        Assert.Equal(
            "SELECT \"id\", \"username\" FROM \"user\" WHERE id = $id",
            PostgresStatementBuilder.CreateQuery("user", ["id", "username"], where).GetText());
        Assert.Equal(
            "INSERT INTO \"user\" (id, username) VALUES ($id, $username)",
            PostgresStatementBuilder.CreateInsert("user", fields).GetText());
        Assert.Equal(
            "UPDATE \"user\" SET id = $id, username = $username WHERE id = $id",
            PostgresStatementBuilder.CreateUpdate("user", fields, where).GetText());
        Assert.Equal(
            "DELETE FROM \"user\" WHERE id = $id",
            PostgresStatementBuilder.CreateDelete("user", where).GetText());
        Assert.Equal(
            "CALL \"save\"($id, $username)",
            PostgresStatementBuilder.CreateStoredProcedure("save", fields).GetText());
    }
}
