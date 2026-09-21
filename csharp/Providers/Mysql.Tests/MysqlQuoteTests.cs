using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlQuoteTests
{
    [Fact]
    public void QuotesAndQualifies()
    {
        Assert.Equal("`user`", MysqlQuote.Quote("user"));
        Assert.Equal("`a``b`", MysqlQuote.Quote("a`b"));
        Assert.Equal("`users`.`id`", MysqlQuote.Qualified("users.id"));
    }

    [Fact]
    public void QuotesTablesInDml()
    {
        var where = new WhereClause(new Operator.Equals("id", 1));
        Parameter[] fields = [new("id", 1), new("username", "alice")];
        Assert.Equal(
            "SELECT `id`, `username` FROM `user` WHERE id = $id",
            new QueryStatement(MysqlQuote.Quote("user"), [MysqlQuote.Quote("id"), MysqlQuote.Quote("username")], where).GetText());
        Assert.Equal(
            "INSERT INTO `user` (id, username) VALUES ($id, $username)",
            new InsertStatement(MysqlQuote.Quote("user"), fields).GetText());
        Assert.Equal(
            "UPDATE `user` SET id = $id, username = $username WHERE id = $id",
            new UpdateStatement(MysqlQuote.Quote("user"), fields, where).GetText());
        Assert.Equal(
            "DELETE FROM `user` WHERE id = $id",
            new DeleteStatement(MysqlQuote.Quote("user"), where).GetText());
        Assert.Equal(
            "CALL `save`($id, $username)",
            new StoredProcedureStatement(MysqlQuote.Quote("save"), fields).GetText());
    }
}
