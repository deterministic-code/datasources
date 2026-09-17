using Datasources;
using Datasources.Builders;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlStatementBuilderTests
{
    static readonly MysqlColumn[] UserColumns =
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
        Assert.Equal("`user`", MysqlStatementBuilder.Quote("user"));
        Assert.Equal("`a``b`", MysqlStatementBuilder.Quote("a`b"));
        Assert.Equal("INT", MysqlStatementBuilder.ColumnType("int"));
        var error = Assert.Throws<DatasourceException>(() => MysqlStatementBuilder.ColumnType("xml"));
        Assert.Contains("Unknown mysql type \"xml\"", error.Message);
    }

    [Fact]
    public void CreatesUserTable() =>
        Assert.Equal(
            "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)",
            MysqlStatementBuilder.CreateTable("user", UserColumns).GetText());

    [Fact]
    public void QuotesTablesInDml()
    {
        var where = WhereBuilder.Equals("id", 1);
        Parameter[] fields = [new("id", 1), new("username", "alice")];
        Assert.Equal(
            "SELECT `id`, `username` FROM `user` WHERE id = $id",
            MysqlStatementBuilder.CreateQuery("user", ["id", "username"], where).GetText());
        Assert.Equal(
            "INSERT INTO `user` (id, username) VALUES ($id, $username)",
            MysqlStatementBuilder.CreateInsert("user", fields).GetText());
        Assert.Equal(
            "UPDATE `user` SET id = $id, username = $username WHERE id = $id",
            MysqlStatementBuilder.CreateUpdate("user", fields, where).GetText());
        Assert.Equal(
            "DELETE FROM `user` WHERE id = $id",
            MysqlStatementBuilder.CreateDelete("user", where).GetText());
        Assert.Equal(
            "CALL `save`($id, $username)",
            MysqlStatementBuilder.CreateStoredProcedure("save", fields).GetText());
    }
}
