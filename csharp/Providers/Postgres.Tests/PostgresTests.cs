using Datasources;
using Datasources.Postgres;

namespace Datasources.Postgres.Tests;

public class PostgresTests
{
    public PostgresTests()
    {
        DatasourceFactory.Reset();
        DatasourceFactory.Register("postgres", new PostgresProvider());
    }

    [Fact]
    public async Task CommandRepositoryAndFactory()
    {
        Assert.Throws<DatasourceException>(() => PostgresGuard.Require(null));
        var driver = new FakePostgresDriver();
        Assert.Same(driver, PostgresGuard.Require(driver));
        var (sql, values) = PostgresBind.ToPostgres("SELECT * FROM t WHERE id = $id AND n = $id", new Row { ["id"] = 1 });
        Assert.Equal("SELECT * FROM t WHERE id = $1 AND n = $1", sql);
        Assert.Equal(new object?[] { 1L }, values);
        var stamp = new DateTime(2020, 1, 2, 3, 4, 5, DateTimeKind.Utc);
        var (_, dates) = PostgresBind.ToPostgres("SELECT $at", new Row { ["at"] = stamp });
        Assert.Equal(new object?[] { "2020-01-02T03:04:05.000Z" }, dates);
        Assert.Contains("Host=127.0.0.1", PostgresConnectionString.ToConnectionString("postgres://ds:ds@127.0.0.1:5432/datasources"));
        Assert.Contains("Username=ds", PostgresConnectionString.ToConnectionString("postgresql://ds@127.0.0.1/datasources"));
        Assert.Equal("Host=db;Port=1", PostgresConnectionString.ToConnectionString("Host=db;Port=1"));
        Assert.Equal("INSERT INTO t (id) VALUES (1) RETURNING id", PostgresBind.WithReturning("INSERT INTO t (id) VALUES (1)"));
        Assert.Equal("INSERT INTO t VALUES (1) RETURNING id", PostgresBind.WithReturning("INSERT INTO t VALUES (1) RETURNING id"));
        Assert.Equal("UPDATE t SET x = 1", PostgresBind.WithReturning("UPDATE t SET x = 1"));
        Assert.Equal("READ UNCOMMITTED", PostgresBind.IsolationSql(IsolationLevel.ReadUncommitted));
        Assert.Equal("READ COMMITTED", PostgresBind.IsolationSql(IsolationLevel.ReadCommitted));
        Assert.Equal("REPEATABLE READ", PostgresBind.IsolationSql(IsolationLevel.RepeatableRead));
        Assert.Equal("SERIALIZABLE", PostgresBind.IsolationSql(IsolationLevel.Serializable));

        var conn = new PostgresConnection(new PostgresConnectionOptions { Url = "postgres://db/app", Schema = "s", Driver = driver });
        await conn.Open();
        await conn.Open();
        Assert.Equal("s", conn.Schema);
        var command = new PostgresCommand(conn);
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

        Assert.Throws<DatasourceException>(() => new PostgresConnection());
        var defaulted = new PostgresConnection(new PostgresConnectionOptions { Driver = driver });
        Assert.Equal("", defaulted.Url);

        var ds = new Datasource("postgres", new Of(() => new PostgresConnection(new PostgresConnectionOptions { Driver = driver })));
        Assert.Equal("postgres", ds.Provider);
        var created = await ds.CreateConnection();
        await ((PostgresConnection)created).Open();
        var bound = Assert.IsType<PostgresCommand>(ds.CreateCommand(created));
        Assert.Same(created, bound.Connection);
        await bound.Execute("");
        await bound.ExecuteReader("SELECT 1");
        var implicitCommand = Assert.IsType<PostgresCommand>(ds.CreateCommand());
        await implicitCommand.Connection.Open();
        await implicitCommand.Execute("");
        await implicitCommand.ExecuteReader("SELECT 1");
        var repo2 = ds.CreateRepository<Dictionary<string, Value>>(created);
        Assert.IsType<CRUDRepository<Dictionary<string, Value>>>(repo2);
        Assert.Throws<DatasourceException>(() =>
            new Datasource("postgres", new PostgresProvider())
                .CreateRepository<Dictionary<string, Value>>());
        Assert.Equal("postgres", DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "postgres" }).Provider);
        await Assert.ThrowsAsync<DatasourceException>(() =>
            new Datasource("postgres", new PostgresProvider()).CreateConnection());

        var closed = new PostgresConnection(new PostgresConnectionOptions { Driver = driver });
        await Assert.ThrowsAsync<DatasourceException>(() => new PostgresCommand(closed).Execute("x"));
        await Assert.ThrowsAsync<DatasourceException>(() => new PostgresCommand(closed).ExecuteReader("x"));

        await driver.Execute("   ", new Row());
        await driver.Query("   ", new Row());
    }

    sealed class FakePostgresDriver : IPostgresDriver
    {
        public Task Open(string url) => Task.CompletedTask;
        public Task Close() => Task.CompletedTask;
        public Task<PostgresExecuteResult> Execute(string sql, Row parameters) =>
            Task.FromResult(string.IsNullOrWhiteSpace(sql) ? new PostgresExecuteResult(0, Value.Null.Instance) : new PostgresExecuteResult(2, 5));
        public Task<PostgresExecuteResult> Execute(string sql, IReadOnlyList<Parameter>? parameters) =>
            Execute(sql, Helpers.ParametersToRecord(parameters));
        public Task<IReadOnlyList<Row>> Query(string sql, Row parameters) =>
            Task.FromResult<IReadOnlyList<Row>>(
                string.IsNullOrWhiteSpace(sql) ? [] : [new Row { ["id"] = 1 }]);
        public Task Begin(IsolationLevel isolation = IsolationLevel.Serializable) => Task.CompletedTask;
        public Task Commit() => Task.CompletedTask;
        public Task Rollback() => Task.CompletedTask;
    }

    sealed class Of(Func<IConnection> connection) : IDriver
    {
        public IConnection Connection(IReadOnlyDictionary<string, string?>? _) => connection();
        public ICommand Command(IConnection c) => c.CreateCommand();
        public ITransaction Transaction(IConnection c, IsolationLevel isolation = IsolationLevel.Serializable) =>
            new PostgresTransaction((PostgresConnection)c, isolation);
        public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters => PostgresRowTypeConverters.All;
    }
}
