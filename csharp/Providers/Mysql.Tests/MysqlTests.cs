using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlTests
{
    public MysqlTests()
    {
        DatasourceFactory.Reset();
        DatasourceFactory.Register("mysql", new MysqlDriver());
    }

    [Fact]
    public async Task CommandRepositoryAndFactory()
    {
        Assert.Equal("SELECT * FROM t WHERE id = @id", MysqlBind.ToMysql("SELECT * FROM t WHERE id = $id"));
        Assert.Equal("INSERT INTO t (id) VALUES (@id)", MysqlBind.ToMysql("INSERT INTO t (id) VALUES ($id) RETURNING id"));
        Assert.Equal(
            "INSERT INTO contact_settings (contact_id, `key`, value) VALUES (@contact_id, @key, @value)",
            MysqlBind.ToMysql("INSERT INTO contact_settings (contact_id, key, value) VALUES ($contact_id, $key, $value)"));
        Assert.Equal("CREATE TABLE t (id INT PRIMARY KEY)", MysqlBind.ToMysql("CREATE TABLE t (id INT PRIMARY KEY)"));
        Assert.Equal(
            "CREATE TABLE t (id INT AUTO_INCREMENT, CONSTRAINT `pk` PRIMARY KEY (`id`))",
            MysqlBind.ToMysql("CREATE TABLE t (id INT AUTO_INCREMENT, CONSTRAINT `pk` PRIMARY KEY (`id`))"));
        Assert.Equal("INSERT INTO `OldContactsTbl` (`CntID`) VALUES (@CntID)", MysqlBind.ToMysql("INSERT INTO OldContactsTbl (CntID) VALUES ($CntID)"));
        Assert.Equal("READ UNCOMMITTED", MysqlBind.IsolationSql(IsolationLevel.ReadUncommitted));
        Assert.Equal("READ COMMITTED", MysqlBind.IsolationSql(IsolationLevel.ReadCommitted));
        Assert.Equal("REPEATABLE READ", MysqlBind.IsolationSql(IsolationLevel.RepeatableRead));
        Assert.Equal("REPEATABLE READ", MysqlBind.IsolationSql(IsolationLevel.Serializable));

        var conn = new MysqlConnection(new MysqlConnectionOptions { Url = "mysql://db/app" });
        Assert.Equal("mysql://db/app", conn.Url);
        var command = new MysqlCommand(conn);
        await command.Execute(new Native(""));
        await command.ExecuteReader(new Native(""));

        Assert.False(conn.IsOpen);
        Assert.Throws<DatasourceException>(() => conn.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => conn.StartTransaction());

        var created = new MysqlConnection();
        Assert.Equal("", created.Url);

        var ds = new Datasource("mysql", new MysqlDriver());
        Assert.Equal("mysql", ds.Provider);
        var provided = Assert.IsType<MysqlConnection>(await ds.CreateConnection());
        var bound = Assert.IsType<MysqlCommand>(await ds.CreateCommand(provided));
        Assert.Same(provided, bound.Connection);
        await bound.Execute(new Native(""));
        await bound.ExecuteReader(new Native(""));
        Assert.IsType<CRUDRepository<Dictionary<string, Value>>>(
            ds.CreateRepository<Dictionary<string, Value>>(provided));
        Assert.Equal("mysql", DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "mysql" }).Provider);

        var closed = new MysqlConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new MysqlCommand(closed).Execute(Native.N("x")));
        await Assert.ThrowsAsync<DatasourceException>(() => new MysqlCommand(closed).ExecuteReader(Native.N("x")));
    }

    [Fact]
    public void MapsColumnTypes()
    {
        var driver = new MysqlDriver();
        Assert.Equal("INT", driver.ColumnType("int"));
        Assert.Equal("CHAR(36)", driver.ColumnType("uuid"));
        Assert.Equal("TEXT", driver.ColumnType("string"));
        Assert.Equal("TEXT", driver.ColumnType("datetime"));
        Assert.Equal("TINYINT", driver.ColumnType("bool"));
        Assert.Equal("DECIMAL(18,4)", driver.ColumnType("decimal"));
        Assert.Equal("DOUBLE", driver.ColumnType("float"));
        Assert.Equal("BLOB", driver.ColumnType("binary"));
        Assert.Equal("(X'ab')", driver.DefaultValueConverter!.Convert("binary", "Hex('ab')"));
        Assert.Equal("Hex('ab')", driver.DefaultValueConverter.ReverseConvert("binary", "(X'ab')"));
        Assert.Equal("Hex('ab')", driver.DefaultValueConverter.ReverseConvert("binary", "X'ab'"));
        Assert.Throws<InvalidOperationException>(() => driver.DefaultValueConverter.ReverseConvert("binary", "X'ab')"));
        Assert.Throws<InvalidOperationException>(() => driver.DefaultValueConverter.ReverseConvert("binary", "(X'ab'"));
        Assert.Equal("('')", driver.DefaultValueConverter.Convert("string", ""));
        Assert.Equal("('hi')", driver.DefaultValueConverter.Convert("string", "hi"));
        Assert.Equal("String", driver.DefaultValueConverter.ReverseConvert("string", "('hi')"));
        Assert.Equal("String", driver.DefaultValueConverter.ReverseConvert("string", "'hi'"));
        Assert.Throws<InvalidOperationException>(() => driver.DefaultValueConverter.ReverseConvert("string", "'hi')"));
        Assert.Throws<InvalidOperationException>(() => driver.DefaultValueConverter.ReverseConvert("string", "('hi'"));
        Assert.Contains("Unknown mysql type \"xml\"", Assert.Throws<DatasourceException>(() => driver.ColumnType("xml")).Message);
    }

    [Fact]
    public void MapsPropertyTargets()
    {
        var converter = RowTypeConverter.From(new MysqlDriver().RowTypeConverters, new Dictionary<string, string>
        {
            ["Key"] = "uuid",
            ["Active"] = "boolean",
            ["Login"] = "string",
            ["Created"] = "datetime",
        });
        var id = Guid.Parse("11111111-1111-1111-1111-111111111111");
        var row = converter.Convert(new Row
        {
            ["Key"] = id.ToString(),
            ["Active"] = 1L,
            ["Login"] = "ada",
            ["Created"] = "2020-01-02T03:04:05.000Z",
        });
        Assert.Equal(new Value.Guid(id), row["Key"]);
        Assert.Equal(new Value.Bool(true), row["Active"]);
        Assert.Equal(new Value.Text("ada"), row["Login"]);
        Assert.True(row["Created"] is Value.DateTime);
        var stored = converter.ReverseConvert(row);
        Assert.Equal(new Value.Text(id.ToString()), stored["Key"]);
        Assert.Equal(new Value.Int(1), stored["Active"]);
        Assert.Equal(new Value.Text("ada"), stored["Login"]);
        Assert.Equal(new Value.Text("2020-01-02 03:04:05.000"), stored["Created"]);
    }
}
