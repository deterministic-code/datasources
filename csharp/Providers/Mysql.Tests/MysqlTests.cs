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
        Assert.Equal("READ UNCOMMITTED", MysqlBind.IsolationSql(IsolationLevel.ReadUncommitted));
        Assert.Equal("READ COMMITTED", MysqlBind.IsolationSql(IsolationLevel.ReadCommitted));
        Assert.Equal("REPEATABLE READ", MysqlBind.IsolationSql(IsolationLevel.RepeatableRead));
        Assert.Equal("REPEATABLE READ", MysqlBind.IsolationSql(IsolationLevel.Serializable));

        var conn = new MysqlConnection(new MysqlConnectionOptions { Url = "mysql://db/app" });
        Assert.Equal("mysql://db/app", conn.Url);
        var command = new MysqlCommand(conn);
        await command.Execute("");
        await command.ExecuteReader("");

        Assert.False(conn.IsOpen);
        Assert.Throws<DatasourceException>(() => conn.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => conn.StartTransaction());

        var created = new MysqlConnection();
        Assert.Equal("", created.Url);

        var ds = new Datasource("mysql", new MysqlDriver());
        Assert.Equal("mysql", ds.Provider);
        var provided = Assert.IsType<MysqlConnection>(await ds.CreateConnection());
        var bound = Assert.IsType<MysqlCommand>(ds.CreateCommand(provided));
        Assert.Same(provided, bound.Connection);
        await bound.Execute("");
        await bound.ExecuteReader("");
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
        Assert.Contains("Unknown mysql type \"xml\"", Assert.Throws<DatasourceException>(() => driver.ColumnType("xml")).Message);
    }

    [Fact]
    public void RowConverterMapsUuidAndBooleanAndLeavesUnknownText()
    {
        IDriver driver = new MysqlDriver();
        var converter = driver.RowConverter(new Dictionary<string, string>
        {
            ["Key"] = "uuid",
            ["Active"] = "TINYINT",
            ["Login"] = "XML",
        });
        var id = Guid.Parse("11111111-1111-1111-1111-111111111111");
        var row = converter.Convert(new Row
        {
            ["Key"] = id.ToString(),
            ["Active"] = 1L,
            ["Login"] = "ada",
        });
        Assert.Equal(new Value.Guid(id), row["Key"]);
        Assert.Equal(new Value.Bool(true), row["Active"]);
        Assert.Equal(new Value.Text("ada"), row["Login"]);
        var stored = converter.ReverseConvert(row);
        Assert.Equal(new Value.Text(id.ToString()), stored["Key"]);
        Assert.Equal(new Value.Int(1), stored["Active"]);
        Assert.Equal(new Value.Text("ada"), stored["Login"]);
    }
}
