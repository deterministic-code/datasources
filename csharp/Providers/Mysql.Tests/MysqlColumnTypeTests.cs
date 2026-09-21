using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlColumnTypeTests
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
    public void MapsKnownAndUnknown()
    {
        Assert.Equal("INT", MysqlColumnType.Map("int"));
        var error = Assert.Throws<DatasourceException>(() => MysqlColumnType.Map("xml"));
        Assert.Contains("Unknown mysql type \"xml\"", error.Message);
    }

    [Fact]
    public void CreatesUserTable() =>
        Assert.Equal(
            "CREATE TABLE `user` (`id` INT PRIMARY KEY, `uuid` CHAR(36), `username` TEXT, `is_enabled` TINYINT, `balance` DECIMAL(18,4), `location` DOUBLE, `created` TEXT, `version` BLOB)",
            new CreateTableStatement(
                MysqlQuote.Quote("user"),
                [.. UserColumns.Select(column => new Column(
                    MysqlQuote.Quote(column.Name),
                    column.Id ? $"{MysqlColumnType.Map(column.Type)} PRIMARY KEY" : MysqlColumnType.Map(column.Type)))]).GetText());
}
