using Datasources;
using Datasources.Mysql;

namespace Datasources.Mysql.Tests;

public class MysqlTests
{
    public MysqlTests()
    {
        Datasource.Reset();
        Datasource.Use(MysqlDatasource.Instance);
    }

    [Fact]
    public async Task CommandRepositoryAndFactory()
    {
        Assert.Throws<DatasourceException>(() => MysqlGuard.Require(null));
        var driver = new FakeMysqlDriver();
        Assert.Same(driver, MysqlGuard.Require(driver));
        Assert.Equal("SELECT * FROM t WHERE id = @id", MysqlBind.ToMysql("SELECT * FROM t WHERE id = $id"));
        Assert.Equal("INSERT INTO t (id) VALUES (@id)", MysqlBind.ToMysql("INSERT INTO t (id) VALUES ($id) RETURNING id"));
        Assert.Equal("READ UNCOMMITTED", MysqlBind.IsolationSql(IsolationLevel.ReadUncommitted));
        Assert.Equal("READ COMMITTED", MysqlBind.IsolationSql(IsolationLevel.ReadCommitted));
        Assert.Equal("REPEATABLE READ", MysqlBind.IsolationSql(IsolationLevel.RepeatableRead));
        Assert.Equal("REPEATABLE READ", MysqlBind.IsolationSql(IsolationLevel.Serializable));

        var conn = new MysqlConnection(new MysqlConnectionOptions { Url = "mysql://db/app", Driver = driver });
        await conn.Open();
        await conn.Open();
        var command = new MysqlCommand(conn);
        await command.Execute("");
        await command.ExecuteReader("SELECT 1");
        await command.ExecuteReader("");

        var txn = await conn.StartTransaction();
        Assert.Equal(IsolationLevel.Serializable, txn.Isolation);
        await txn.Commit();
        var dirty = await conn.StartTransaction(IsolationLevel.ReadUncommitted);
        Assert.Equal(IsolationLevel.ReadUncommitted, dirty.Isolation);
        await dirty.Abort();
        await conn.Close();
        Assert.Throws<DatasourceException>(() => conn.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => conn.StartTransaction());

        Assert.Throws<DatasourceException>(() => new MysqlConnection());
        var defaulted = new MysqlConnection(new MysqlConnectionOptions { Driver = driver });
        Assert.Equal("", defaulted.Url);

        var ds = new MysqlDatasource(new MysqlConnectionOptions { Driver = driver }, "people");
        Assert.Equal("mysql", ds.Create().Provider);
        var created = await ds.CreateConnection();
        await ((MysqlConnection)created).Open();
        var bound = Assert.IsType<MysqlCommand>(ds.CreateCommand(created));
        Assert.Same(created, bound.Connection);
        await bound.Execute("");
        await bound.ExecuteReader("SELECT 1");
        var implicitCommand = Assert.IsType<MysqlCommand>(ds.CreateCommand());
        await implicitCommand.Connection.Open();
        await implicitCommand.Execute("");
        await implicitCommand.ExecuteReader("SELECT 1");
        var repo2 = ds.CreateRepository<Dictionary<string, Value>>(created);
        Assert.IsType<Repository<Dictionary<string, Value>>>(repo2);
        Assert.Throws<DatasourceException>(() =>
            new MysqlDatasource().CreateRepository<Dictionary<string, Value>>());
        Assert.Equal("mysql", Datasource.Create("mysql").Provider);
        await Assert.ThrowsAsync<DatasourceException>(() => new MysqlDatasource().CreateConnection());

        var closed = new MysqlConnection(new MysqlConnectionOptions { Driver = driver });
        await Assert.ThrowsAsync<DatasourceException>(() => new MysqlCommand(closed).Execute("x"));
        await Assert.ThrowsAsync<DatasourceException>(() => new MysqlCommand(closed).ExecuteReader("x"));

        await driver.Execute("   ", new Row());
        await driver.Query("   ", new Row());
    }

    sealed class FakeMysqlDriver : IMysqlDriver
    {
        public bool Opened { get; private set; }
        public Task Open(string url) { Opened = true; return Task.CompletedTask; }
        public Task Close() { Opened = false; return Task.CompletedTask; }
        public Task<MysqlExecuteResult> Execute(string sql, Row parameters) =>
            Task.FromResult(string.IsNullOrWhiteSpace(sql) ? new MysqlExecuteResult(0, Value.Null.Instance) : new MysqlExecuteResult(2, 5));
        public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) =>
            Task.FromResult<IReadOnlyList<Row>>(
                string.IsNullOrWhiteSpace(sql) ? [] : [new Row { ["id"] = 1 }]);
        public Task Begin(IsolationLevel isolation = IsolationLevel.Serializable) => Task.CompletedTask;
        public Task Commit() => Task.CompletedTask;
        public Task Rollback() => Task.CompletedTask;
    }
}
