using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteQuoteTests
{
    [Fact]
    public void QuotesAndQualifies()
    {
        Assert.Equal("\"user\"", SqliteQuote.Quote("user"));
        Assert.Equal("\"a\"\"b\"", SqliteQuote.Quote("a\"b"));
        Assert.Equal("\"users\".\"id\"", SqliteQuote.Qualified("users.id"));
    }

    [Fact]
    public void QuotesTablesInDml()
    {
        var where = new WhereClause(new Operator.Equals("id", 1));
        Parameter[] fields = [new("id", 1), new("username", "alice")];
        Assert.Equal(
            "SELECT \"id\", \"username\" FROM \"user\" WHERE id = $id",
            new QueryStatement(SqliteQuote.Quote("user"), [SqliteQuote.Quote("id"), SqliteQuote.Quote("username")], where).GetText());
        Assert.Equal(
            "INSERT INTO \"user\" (id, username) VALUES ($id, $username)",
            new InsertStatement(SqliteQuote.Quote("user"), fields).GetText());
        Assert.Equal(
            "UPDATE \"user\" SET id = $id, username = $username WHERE id = $id",
            new UpdateStatement(SqliteQuote.Quote("user"), fields, where).GetText());
        Assert.Equal(
            "DELETE FROM \"user\" WHERE id = $id",
            new DeleteStatement(SqliteQuote.Quote("user"), where).GetText());
        Assert.Equal(
            "CALL \"save\"($id, $username)",
            new StoredProcedureStatement(SqliteQuote.Quote("save"), fields).GetText());
        Assert.Equal(
            "SELECT \"users\".\"id\", \"orders\".\"total\" FROM \"users\" INNER JOIN \"orders\" ON \"users\".\"id\" = \"orders\".\"user_id\" LEFT JOIN \"payments\" ON \"orders\".\"id\" = \"payments\".\"order_id\" WHERE users.id = $users.id",
            new QueryStatement(
                SqliteQuote.Quote("users"),
                [SqliteQuote.Qualified("users.id"), SqliteQuote.Qualified("orders.total")],
                new WhereClause(new Operator.Equals("users.id", 1)),
                [
                    Join.Inner(SqliteQuote.Qualified("orders"), SqliteQuote.Qualified("users.id"), SqliteQuote.Qualified("orders.user_id")),
                    Join.Left(SqliteQuote.Qualified("payments"), SqliteQuote.Qualified("orders.id"), SqliteQuote.Qualified("payments.order_id")),
                ]).GetText());
    }
}
