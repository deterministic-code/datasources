using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

[Collection("DatasourceFactory")]
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
        await command.Execute(new Native(""));
        await command.ExecuteReader(Native.N("SELECT 1"));
        await command.ExecuteReader(new Native(""));

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
        var bound = Assert.IsType<SqliteCommand>(await ds.CreateCommand(provided));
        Assert.Same(provided, bound.Connection);
        await bound.Execute(new Native(""));
        await bound.ExecuteReader(Native.N("SELECT 1"));
        Assert.IsType<CRUDRepository<Dictionary<string, Value>>>(
            ds.CreateRepository<Dictionary<string, Value>>(provided));
        Assert.Equal("sqlite", DatasourceFactory.Create(new Dictionary<string, string?> { ["PROVIDER"] = "sqlite" }).Provider);

        var closed = new SqliteConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).Execute(Native.N("x")));
        await Assert.ThrowsAsync<DatasourceException>(() => new SqliteCommand(closed).ExecuteReader(Native.N("x")));
    }

    [Fact]
    public void ParsesConnectionStringOptions()
    {
        var parsed = SqlitePath.Parse("sqlite:///./app.sqlite?journal_mode=delete&busy_timeout=1000&pool_size=4&synchronous=normal");
        Assert.NotNull(parsed);
        Assert.Equal("./app.sqlite", parsed.Value.Filename);
        Assert.Equal("DELETE", parsed.Value.Options.JournalMode);
        Assert.Equal(1000, parsed.Value.Options.BusyTimeout);
        Assert.Equal(4, parsed.Value.Options.PoolSize);
        Assert.Equal("NORMAL", parsed.Value.Options.Synchronous);
        var defaults = SqliteOptions.Empty;
        Assert.Equal("WAL", defaults.ResolvedJournalMode("app.sqlite"));
        Assert.Equal(5000, defaults.ResolvedBusyTimeout());
        Assert.Equal(8, defaults.ResolvedPoolSize("app.sqlite"));
        Assert.Null(defaults.ResolvedJournalMode(":memory:"));
        Assert.Equal(0, defaults.ResolvedPoolSize(":memory:"));
        Assert.Equal(0, new SqliteOptions { JournalMode = "DELETE" }.ResolvedPoolSize("app.sqlite"));
        Assert.Equal(0, new SqliteOptions { PoolSize = 0 }.ResolvedPoolSize("app.sqlite"));
        var uri = SqlitePath.Parse("sqlite:///file:x?mode=memory&journal_mode=delete&cache=shared");
        Assert.Equal("file:x?mode=memory&cache=shared", uri!.Value.Filename);
        Assert.Contains("journal_mode=fast", SqlitePath.Parse("sqlite:///app.db?journal_mode=fast")!.Value.Options.Error);
        Assert.Contains("pool_size=65", SqlitePath.ParseFilenameQuery("app.db?pool_size=65").Options.Error);
    }

    [Fact]
    public async Task BadOptionFailsOpen()
    {
        var connection = new SqliteConnection(new Dictionary<string, string?>
        {
            ["DATASOURCE_URL"] = "sqlite:///app.db?journal_mode=fast",
        });
        var error = await Assert.ThrowsAsync<DatasourceException>(connection.Open);
        Assert.Contains("journal_mode=fast", error.Message);
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

    [Fact]
    public void MapsPropertyTargets()
    {
        var converter = RowTypeConverter.From(new SqliteDriver().RowTypeConverters, new Dictionary<string, string>
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
        Assert.Equal(new Value.Text(id.ToString()), row["Key"]);
        Assert.Equal(new Value.Bool(true), row["Active"]);
        Assert.Equal(new Value.Text("ada"), row["Login"]);
        Assert.True(row["Created"] is Value.DateTime);
        var stored = converter.ReverseConvert(row);
        Assert.Equal(new Value.Text(id.ToString()), stored["Key"]);
        Assert.Equal(new Value.Int(1), stored["Active"]);
        Assert.Equal(new Value.Text("ada"), stored["Login"]);
        Assert.Equal(new Value.Text("2020-01-02T03:04:05.000Z"), stored["Created"]);
    }
}
