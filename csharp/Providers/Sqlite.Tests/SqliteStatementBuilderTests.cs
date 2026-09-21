using Datasources;
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
        var where = new WhereClause(new Operator.Equals("id", 1));
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
        Assert.Equal(
            "CALL \"save\"($id, $username)",
            SqliteStatementBuilder.CreateStoredProcedure("save", fields).GetText());
        Assert.Equal(
            "SELECT \"users\".\"id\", \"orders\".\"total\" FROM \"users\" INNER JOIN \"orders\" ON \"users\".\"id\" = \"orders\".\"user_id\" LEFT JOIN \"payments\" ON \"orders\".\"id\" = \"payments\".\"order_id\" WHERE users.id = $users.id",
            SqliteStatementBuilder.CreateQuery(
                "users",
                ["users.id", "orders.total"],
                new WhereClause(new Operator.Equals("users.id", 1)),
                [
                    Join.Inner("orders", "users.id", "orders.user_id"),
                    Join.Left("payments", "orders.id", "payments.order_id"),
                ]).GetText());
    }
}
