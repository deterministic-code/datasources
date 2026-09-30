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
        var command = new SqliteCommand(conn);
        await command.Execute("");
        await command.ExecuteReader(Native.N("SELECT 1"));
        await command.ExecuteReader("");

        var txn = await conn.StartTransaction();
        Assert.Equal(IsolationLevel.ReadCommitted, txn.Isolation);
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
        await bound.ExecuteReader(Native.N("SELECT 1"));
        Assert.IsType<CRUDRepository<Dictionary<string, Value>>>(
            ds.CreateRepository<Dictionary<string, Value>>(provided));
        Assert.Equal("sqlite", DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "sqlite" }).Provider);

        var closed = new SqliteConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).Execute(Native.N("x")));
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).ExecuteReader(Native.N("x")));
    }

    [Fact]
    public void MapsColumnTypes()
    {
        var driver = new SqliteDriver();
        Assert.Equal("INTEGER", driver.ColumnType("int"));
        Assert.Equal("TEXT", driver.ColumnType("uuid"));
        Assert.Equal("TEXT", driver.ColumnType("string"));
        Assert.Equal("BOOLEAN", driver.ColumnType("bool"));
        Assert.Equal("NUMERIC", driver.ColumnType("decimal"));
        Assert.Equal("REAL", driver.ColumnType("float"));
        Assert.Equal("TEXT", driver.ColumnType("datetime"));
        Assert.Equal("BLOB", driver.ColumnType("binary"));
        Assert.Contains("Unknown sqlite type \"xml\"", Assert.Throws<DatasourceException>(() => driver.ColumnType("xml")).Message);
    }
}
