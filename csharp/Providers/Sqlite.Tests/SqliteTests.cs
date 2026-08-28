using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteTests
{
    public SqliteTests()
    {
        Datasource.Reset();
        Datasource.Use(SqliteDatasource.Factory);
    }

    [Fact]
    public async Task CommandRepositoryAndFactory()
    {
        Assert.Throws<DatasourceException>(() => SqliteGuard.Require(null));
        var driver = new SqliteDriver();
        Assert.Same(driver, SqliteGuard.Require(driver));

        var conn = new SqliteConnection(new SqliteConnectionOptions { Driver = driver });
        await conn.Open();
        await conn.Open();
        var command = new SqliteCommand(conn);
        await command.Execute();
        await command.ExecuteReader("SELECT 1");
        await command.ExecuteReader();

        var repo = new SqliteRepository(conn);
        Assert.Equal("items", repo.Table);
        Assert.Same(conn, repo.Connection);
        var named = new SqliteRepository(conn, "users");
        Assert.Equal("users", named.Table);

        var txn = await conn.StartTransaction();
        Assert.Equal(IsolationLevel.Serializable, txn.Isolation);
        await txn.Commit();
        var dirty = await conn.StartTransaction(IsolationLevel.ReadUncommitted);
        Assert.Equal(IsolationLevel.ReadUncommitted, dirty.Isolation);
        await dirty.Abort();
        var txn2 = await conn.StartTransaction();
        await txn2.Abort();
        await conn.Close();
        Assert.Throws<DatasourceException>(() => conn.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => conn.StartTransaction());

        var shared = new SqliteConnection(new SqliteConnectionOptions
        {
            Filename = "file:iso?mode=memory&cache=shared",
            Driver = new SqliteDriver()
        });
        await shared.Open();
        var readCommitted = await shared.StartTransaction(IsolationLevel.ReadCommitted);
        Assert.Equal(IsolationLevel.ReadCommitted, readCommitted.Isolation);
        await readCommitted.Abort();
        await shared.Close();

        var defaulted = new SqliteConnection();
        Assert.Equal(":memory:", defaulted.Filename);
        Assert.IsType<SqliteDriver>(defaulted.Driver);

        var ds = new SqliteDatasource(new SqliteConnectionOptions { Driver = driver }, "people");
        Assert.Equal("sqlite", ds.Create().Provider);
        var created = await ds.CreateConnection();
        await ((SqliteConnection)created).Open();
        var bound = Assert.IsType<SqliteCommand>(ds.CreateCommand(created));
        Assert.Same(created, bound.Connection);
        await bound.Execute();
        await bound.ExecuteReader("SELECT 1");
        var implicitCommand = Assert.IsType<SqliteCommand>(ds.CreateCommand());
        await implicitCommand.Connection.Open();
        await implicitCommand.Execute();
        await implicitCommand.ExecuteReader("SELECT 1");
        var repo2 = ds.CreateRepository<Dictionary<string, object?>>(created);
        Assert.Equal("people", ((SqliteRepository)(object)repo2).Table);
        new SqliteDatasource().CreateRepository<Dictionary<string, object?>>();
        Assert.Equal("sqlite", Datasource.Create("sqlite").Provider);
        Assert.IsType<SqliteConnection>(await new SqliteDatasource().CreateConnection());

        var closed = new SqliteConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).Execute("x"));
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).ExecuteReader("x"));
    }
}
