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
        var driver = new ScriptedDriver
        {
            QueryResult = { new Dictionary<string, object?> { ["id"] = 1 } },
        };
        Assert.Same(driver, SqliteGuard.Require(driver));

        Assert.Throws<DatasourceException>(() => new SqliteConnection());
        var conn = new SqliteConnection(new SqliteConnectionOptions { Filename = "x.db", Driver = driver });
        await conn.Open();
        await conn.Open();
        var command = new SqliteCommand(conn);
        await command.Execute("DELETE FROM items", new[] { new Parameter("id", 1) });
        await command.Execute();
        await command.ExecuteReader("SELECT * FROM items");
        await command.ExecuteReader();

        var repo = new SqliteRepository(conn);
        Assert.NotNull(await repo.GetSingle());
        driver.QueryResult = new List<IReadOnlyDictionary<string, object?>>();
        Assert.Null(await repo.GetSingle(new Query("id = 1")));
        driver.QueryResult = new List<IReadOnlyDictionary<string, object?>> { new Dictionary<string, object?> { ["id"] = 1 } };
        Assert.NotEmpty(await repo.Get().ToListAsync());
        await repo.Add(new Dictionary<string, object?> { ["id"] = 2 });
        await repo.Add(new Dictionary<string, object?>());
        await repo.Update(new Dictionary<string, object?> { ["id"] = 2 });
        await repo.Update(new Dictionary<string, object?>(), new Where("id = 1"));
        await repo.Delete(new Dictionary<string, object?> { ["id"] = 2 });
        await repo.Delete(new Dictionary<string, object?> { ["id"] = 2 }, new Where("name = $name"));
        var named = new SqliteRepository(conn, "users");
        await named.Get().ToListAsync();
        Assert.Equal("users", named.Table);

        var txn = await conn.StartTransaction();
        await txn.Commit();
        var txn2 = await conn.StartTransaction();
        await txn2.Abort();
        await conn.Close();
        Assert.Throws<DatasourceException>(() => conn.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => conn.StartTransaction());

        var defaulted = new SqliteConnection(new SqliteConnectionOptions { Driver = driver });
        Assert.Equal(":memory:", defaulted.Filename);

        var ds = new SqliteDatasource(new SqliteConnectionOptions { Driver = driver }, "people");
        Assert.Equal("sqlite", ds.Create().Provider);
        var created = await ds.CreateConnection();
        await ((SqliteConnection)created).Open();
        ds.CreateCommand(created);
        ds.CreateCommand();
        var repo2 = ds.CreateRepository<Dictionary<string, object?>>(created, new RepositoryOptions { Table = "accounts" });
        Assert.Equal("accounts", ((SqliteRepository)(object)repo2).Table);
        Assert.Equal("people", ((SqliteRepository)(object)ds.CreateRepository<Dictionary<string, object?>>(created)).Table);
        new SqliteDatasource(new SqliteConnectionOptions { Driver = driver }).CreateRepository<Dictionary<string, object?>>();
        Assert.Equal("sqlite", Datasource.Create("sqlite").Provider);
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteDatasource().CreateConnection());

        var closed = new SqliteConnection(new SqliteConnectionOptions { Driver = driver });
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).Execute("x"));
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).ExecuteReader("x"));
    }
}
