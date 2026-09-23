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
        Assert.Equal("mysql", DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "mysql" }).Provider);

        var closed = new MysqlConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new MysqlCommand(closed).Execute("x"));
        await Assert.ThrowsAsync<DatasourceException>(() => new MysqlCommand(closed).ExecuteReader("x"));
    }
}
