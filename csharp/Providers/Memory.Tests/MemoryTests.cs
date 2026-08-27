using Datasources;
using Datasources.Memory;

namespace Datasources.Memory.Tests;

public class MemoryTests
{
    public MemoryTests()
    {
        Datasource.Reset();
        Datasource.Use(MemoryDatasource.Factory);
    }

    [Fact]
    public void FilterHelpers()
    {
        Assert.Empty(MemoryCommand.ParseFilter(null));
        Assert.Empty(MemoryCommand.ParseFilter(""));
        Assert.Equal(1, Convert.ToInt32(MemoryCommand.ParseFilter("{\"id\":1}")["id"]));
        Assert.Throws<DatasourceException>(() => MemoryCommand.ParseFilter("[]"));
        Assert.Throws<DatasourceException>(() => MemoryCommand.ParseFilter("1"));
        Assert.True(MemoryCommand.Matches(new Dictionary<string, object?> { ["id"] = 1 }, new Dictionary<string, object?>()));
        Assert.False(MemoryCommand.Matches(new Dictionary<string, object?>(), new Dictionary<string, object?> { ["id"] = 1 }));
        Assert.IsType<System.Text.Json.Nodes.JsonArray>(MemoryCommand.JsonValueToObject(new System.Text.Json.Nodes.JsonArray()));
        Assert.True((bool)MemoryCommand.JsonValueToObject(System.Text.Json.Nodes.JsonValue.Create(true))!);
        Assert.Equal(1L, Convert.ToInt64(MemoryCommand.JsonValueToObject(System.Text.Json.Nodes.JsonValue.Create(1L))));
        Assert.Equal(1.5, MemoryCommand.JsonValueToObject(System.Text.Json.Nodes.JsonValue.Create(1.5)));
        Assert.Equal("x", MemoryCommand.JsonValueToObject(System.Text.Json.Nodes.JsonValue.Create("x")));
    }

    [Fact]
    public async Task StoreAndConnection()
    {
        var store = new MemoryStore();
        store.Seed(new List<Dictionary<string, object?>> { new() { ["id"] = 1 } }, "");
        store.Seed(new List<Dictionary<string, object?>> { new() { ["id"] = 2 } }, "");
        store.Seed(new Dictionary<string, List<Dictionary<string, object?>>>
        {
            ["users"] = [new() { ["id"] = 3 }],
            ["posts"] = [new() { ["id"] = 4 }],
        }, "");
        store.Seed(new Dictionary<string, List<Dictionary<string, object?>>>
        {
            ["users"] = [new() { ["id"] = 9 }],
        }, "");
        Assert.Single(store.GetCollection(""));
        Assert.Single(store.GetCollection("users"));
        Assert.Equal(3, store.GetCollection("users")[0]["id"]);

        var conn = new MemoryConnection(new MemoryConnectionOptions
        {
            Store = store,
            Data = new List<Dictionary<string, object?>> { new() { ["id"] = 8 } },
        });
        await conn.Open();
        await conn.Open();
        Assert.Single(conn.GetCollection());
        await conn.Close();
        Assert.Throws<DatasourceException>(() => conn.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryConnection().StartTransaction());

        var named = new MemoryConnection(new MemoryConnectionOptions
        {
            Collection = "users",
            Data = new Dictionary<string, List<Dictionary<string, object?>>>
            {
                ["users"] = [new() { ["name"] = "a" }],
            },
        });
        await named.Open();
        Assert.Equal("a", named.GetCollection()[0]["name"]);

        var empty = new MemoryConnection();
        await empty.Open();
        Assert.Empty(empty.GetCollection());
    }

    [Fact]
    public async Task TransactionsAndCrud()
    {
        var conn = new MemoryConnection(new MemoryConnectionOptions
        {
            Data = new List<Dictionary<string, object?>> { new() { ["id"] = 1, ["name"] = "a" } },
        });
        await conn.Open();
        var txn = await conn.StartTransaction();
        conn.GetCollection().Add(new Dictionary<string, object?> { ["id"] = 2 });
        await txn.Abort();
        Assert.Single(conn.GetCollection());

        var committed = await conn.StartTransaction();
        conn.GetCollection().Add(new Dictionary<string, object?> { ["id"] = 2 });
        await committed.Commit();
        Assert.Equal(2, conn.GetCollection().Count);

        var repo = new MemoryRepository(conn);
        var command = new MemoryCommand(conn);
        Assert.NotNull(await repo.GetSingle());
        Assert.Null(await repo.GetSingle(new Query("{\"id\":9}")));
        Assert.NotEmpty(await repo.Get(new Query("{\"id\":1}")).ToListAsync());
        await repo.Add(new Dictionary<string, object?> { ["id"] = 4, ["name"] = "d" });
        await repo.Update(new Dictionary<string, object?> { ["id"] = 4, ["name"] = "D" });
        await repo.Update(new Dictionary<string, object?> { ["name"] = "Z" }, new Where("{\"id\":4}"));
        await repo.Delete(new Dictionary<string, object?> { ["id"] = 4 });
        await command.Execute("insert", [new Parameter("id", 5)]);
        await command.Execute("update", [new Parameter("id", 5), new Parameter("$where", "{\"id\":5}")]);
        await command.Execute("delete", [new Parameter("id", 5)]);
        await command.Execute("delete");
        await Assert.ThrowsAsync<DatasourceException>(() => command.Execute("nope"));
        await Assert.ThrowsAsync<DatasourceException>(() => command.Execute());
        await command.ExecuteReader(null, [new Parameter("name", "a")]);

        var closed = new MemoryConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryCommand(closed).ExecuteReader());
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryCommand(closed).Execute("insert"));
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryRepository(closed).Add(new Dictionary<string, object?>()));
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryRepository(closed).Update(new Dictionary<string, object?> { ["id"] = 1 }));
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryRepository(closed).Delete(new Dictionary<string, object?> { ["id"] = 1 }));

        var ds = new MemoryDatasource(new MemoryConnectionOptions { Data = new List<Dictionary<string, object?>>() });
        Assert.Equal("memory", ds.Create().Provider);
        Assert.Equal("memory", MemoryDatasource.Factory.Provider);
        var created = ds.CreateConnection();
        await ((MemoryConnection)created).Open();
        ds.CreateCommand(created);
        ds.CreateCommand();
        ds.CreateRepository<Dictionary<string, object?>>(created, new RepositoryOptions { Collection = "users" });
        ds.CreateRepository<Dictionary<string, object?>>();
        Assert.Equal("memory", Datasource.Create("memory").Provider);

        var shared = new MemoryStore();
        var a = new MemoryConnection(new MemoryConnectionOptions { Store = shared, Data = new List<Dictionary<string, object?>> { new() { ["id"] = 1 } } });
        var b = new MemoryConnection(new MemoryConnectionOptions { Store = shared });
        await a.Open();
        await b.Open();
        Assert.Single(b.GetCollection());
    }
}
