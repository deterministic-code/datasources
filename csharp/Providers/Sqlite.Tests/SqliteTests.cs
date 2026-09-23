using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class SqliteTests
{
    public SqliteTests()
    {
        DatasourceFactory.Reset();
        DatasourceFactory.Register("sqlite", new SqliteDriver());
    }

    [Fact]
    public async Task CommandRepositoryAndFactory()
    {
        var conn = new SqliteConnection();
        Assert.Equal(":memory:", conn.Filename);
        await conn.Open();
        await conn.Open();
        var command = new SqliteCommand(conn);
        await command.Execute("");
        await command.ExecuteReader("SELECT 1");
        await command.ExecuteReader("");

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
            Filename = "file:iso?mode=memory&cache=shared"
        });
        await shared.Open();
        var readCommitted = await shared.StartTransaction(IsolationLevel.ReadCommitted);
        Assert.Equal(IsolationLevel.ReadCommitted, readCommitted.Isolation);
        await readCommitted.Abort();
        await shared.Close();

        var ds = new Datasource("sqlite", new SqliteDriver());
        Assert.Equal("sqlite", ds.Provider);
        var provided = Assert.IsType<SqliteConnection>(await ds.CreateConnection());
        Assert.Equal(":memory:", provided.Filename);
        await provided.Open();
        var bound = Assert.IsType<SqliteCommand>(ds.CreateCommand(provided));
        Assert.Same(provided, bound.Connection);
        await bound.Execute("");
        await bound.ExecuteReader("SELECT 1");
        Assert.IsType<CRUDRepository<Dictionary<string, Value>>>(
            ds.CreateRepository<Dictionary<string, Value>>(provided));
        Assert.Equal("sqlite", DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "sqlite" }).Provider);

        var closed = new SqliteConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).Execute("x"));
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).ExecuteReader("x"));
    }
}
