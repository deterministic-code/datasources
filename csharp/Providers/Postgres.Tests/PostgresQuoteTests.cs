using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresQuoteTests
{
    [Fact]
    public void QuotesAndQualifies()
    {
        Assert.Equal("\"user\"", PostgresQuote.Quote("user"));
        Assert.Equal("\"a\"\"b\"", PostgresQuote.Quote("a\"b"));
        Assert.Equal("\"users\".\"id\"", PostgresQuote.Qualified("users.id"));
    }

    [Fact]
    public void QuotesTablesInDml()
    {
        var where = new WhereClause(new Operator.Equals("id", 1));
        Parameter[] fields = [new("id", 1), new("username", "alice")];
        Assert.Equal(
            "SELECT \"id\", \"username\" FROM \"user\" WHERE id = $id",
            new QueryStatement(PostgresQuote.Quote("user"), [PostgresQuote.Quote("id"), PostgresQuote.Quote("username")], where).GetText());
        Assert.Equal(
            "INSERT INTO \"user\" (id, username) VALUES ($id, $username)",
            new InsertStatement(PostgresQuote.Quote("user"), fields).GetText());
        Assert.Equal(
            "UPDATE \"user\" SET id = $id, username = $username WHERE id = $id",
            new UpdateStatement(PostgresQuote.Quote("user"), fields, where).GetText());
        Assert.Equal(
            "DELETE FROM \"user\" WHERE id = $id",
            new DeleteStatement(PostgresQuote.Quote("user"), where).GetText());
        Assert.Equal(
            "CALL \"save\"($id, $username)",
            new StoredProcedureStatement(PostgresQuote.Quote("save"), fields).GetText());
    }
}
