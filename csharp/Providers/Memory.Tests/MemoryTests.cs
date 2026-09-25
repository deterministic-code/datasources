using Datasources;
using Datasources.Memory;

namespace Datasources.Memory.Tests;

public class MemoryTests
{
    public MemoryTests()
    {
        DatasourceFactory.Reset();
        DatasourceFactory.Register("memory", new MemoryProvider());
    }

    [Fact]
    public void FilterHelpers()
    {
        Assert.Empty(MemoryCommand.ParseFilter(null));
        Assert.Empty(MemoryCommand.ParseFilter(""));
        Assert.Equal(new Value.Int(1), MemoryCommand.ParseFilter("{\"id\":1}")["id"]);
        Assert.Throws<DatasourceException>(() => MemoryCommand.ParseFilter("[]"));
        Assert.Throws<DatasourceException>(() => MemoryCommand.ParseFilter("1"));
        Assert.True(MemoryCommand.Matches(new Row { ["id"] = 1 }, new Row()));
        Assert.False(MemoryCommand.Matches(new Row(), new Row { ["id"] = 1 }));
        Assert.IsType<Value.Text>(MemoryCommand.JsonValueToValue(new System.Text.Json.Nodes.JsonArray()));
        Assert.Equal(new Value.Bool(true), MemoryCommand.JsonValueToValue(System.Text.Json.Nodes.JsonValue.Create(true)));
        Assert.Equal(new Value.Int(1), MemoryCommand.JsonValueToValue(System.Text.Json.Nodes.JsonValue.Create(1L)));
        Assert.Equal(new Value.Float(1.5), MemoryCommand.JsonValueToValue(System.Text.Json.Nodes.JsonValue.Create(1.5)));
        Assert.Equal(new Value.Text("x"), MemoryCommand.JsonValueToValue(System.Text.Json.Nodes.JsonValue.Create("x")));
    }

    [Fact]
    public async Task StoreAndConnection()
    {
        var store = new MemoryStore();
        store.Seed(new List<Dictionary<string, Value>> { new() { ["id"] = 1 } }, "");
        store.Seed(new List<Dictionary<string, Value>> { new() { ["id"] = 2 } }, "");
        store.Seed(new Dictionary<string, List<Dictionary<string, Value>>>
        {
            ["users"] = [new() { ["id"] = 3 }],
            ["posts"] = [new() { ["id"] = 4 }],
        }, "");
        store.Seed(new Dictionary<string, List<Dictionary<string, Value>>>
        {
            ["users"] = [new() { ["id"] = 9 }],
        }, "");
        Assert.Single(store.GetCollection(""));
        Assert.Single(store.GetCollection("users"));
        Assert.Equal(new Value.Int(3), store.GetCollection("users")[0]["id"]);

        var conn = new MemoryConnection(new MemoryConnectionOptions
        {
            Store = store,
            Data = new List<Dictionary<string, Value>> { new() { ["id"] = 8 } },
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
            Data = new Dictionary<string, List<Dictionary<string, Value>>>
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
            Data = new List<Dictionary<string, Value>> { new() { ["id"] = 1, ["name"] = "a" } },
        });
        await conn.Open();
        var txn = await conn.StartTransaction();
        conn.GetCollection().Add(new Row { ["id"] = 2 });
        await txn.Abort();
        Assert.Single(conn.GetCollection());

        var committed = await conn.StartTransaction();
        conn.GetCollection().Add(new Row { ["id"] = 2 });
        await committed.Commit();
        Assert.Equal(2, conn.GetCollection().Count);

        var repo = new CRUDRepository<Dictionary<string, Value>>(conn);
        var command = new MemoryCommand(conn);
        Assert.NotNull(await repo.ReadSingle());
        Assert.Null(await repo.ReadSingle(new SelectStatement("{\"id\":9}")));
        Assert.NotEmpty(await repo.Read(new SelectStatement("{\"id\":1}")).ToListAsync());
        Assert.NotNull(await repo.ReadSingle(new WhereText("{\"id\":1}")));
        Assert.NotEmpty(await repo.Read(new WhereText("{\"id\":1}")).ToListAsync());
        await repo.Create(new Row { ["id"] = 4, ["name"] = "d" });
        await repo.Update(new Row { ["id"] = 4, ["name"] = "D" });
        await repo.Update(new Row { ["name"] = "Z" }, new WhereText("{\"id\":4}"));
        await repo.Delete(new Row { ["id"] = 4 });
        await command.Execute("insert", [new Parameter("id", 5)]);
        await command.Execute("update", [new Parameter("id", 5), new Parameter("$where", "{\"id\":5}")]);
        await command.Execute("delete", [new Parameter("id", 5)]);
        await command.Execute("delete");
        await Assert.ThrowsAsync<DatasourceException>(() => command.Execute("nope"));
        await Assert.ThrowsAsync<DatasourceException>(() => command.Execute(""));
        await command.ExecuteReader("", [new Parameter("name", "a")]);

        var closed = new MemoryConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryCommand(closed).ExecuteReader(""));
        await Assert.ThrowsAsync<DatasourceException>(() => new MemoryCommand(closed).Execute("insert"));
        await Assert.ThrowsAsync<DatasourceException>(() => new CRUDRepository<Dictionary<string, Value>>(closed).Create(new Row()));
        await Assert.ThrowsAsync<DatasourceException>(() => new CRUDRepository<Dictionary<string, Value>>(closed).Update(new Row { ["id"] = 1 }));
        await Assert.ThrowsAsync<DatasourceException>(() => new CRUDRepository<Dictionary<string, Value>>(closed).Delete(new Row { ["id"] = 1 }));

        var ds = new Datasource("memory", new Of(() => new MemoryConnection(new MemoryConnectionOptions { Data = new List<Dictionary<string, Value>>() })));
        Assert.Equal("memory", ds.Provider);
        var created = await ds.CreateConnection();
        await ((MemoryConnection)created).Open();
        var bound = Assert.IsType<MemoryCommand>(ds.CreateCommand(created));
        Assert.Same(created, bound.Connection);
        await bound.Execute("insert", [new Parameter("id", 99)]);
        await bound.ExecuteReader("");
        var implicitCommand = Assert.IsType<MemoryCommand>(ds.CreateCommand());
        await implicitCommand.Connection.Open();
        await implicitCommand.Execute("insert", [new Parameter("id", 1)]);
        await implicitCommand.ExecuteReader("");
        ds.CreateRepository<Dictionary<string, Value>>(created);
        ds.CreateRepository<Dictionary<string, Value>>();
        Assert.Equal("memory", DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "memory" }).Provider);

        var shared = new MemoryStore();
        var a = new MemoryConnection(new MemoryConnectionOptions { Store = shared, Data = new List<Dictionary<string, Value>> { new() { ["id"] = 1 } } });
        var b = new MemoryConnection(new MemoryConnectionOptions { Store = shared });
        await a.Open();
        await b.Open();
        Assert.Single(b.GetCollection());
    }

    sealed class Of(Func<IConnection> connection) : IDriver
    {
        public IConnection Connection(IReadOnlyDictionary<string, string?>? _) => connection();
        public ICommand Command(IConnection c) => c.CreateCommand();
        public ITransaction Transaction(IConnection c, IsolationLevel isolation = IsolationLevel.Serializable) =>
            new MemoryTransaction((MemoryConnection)c, isolation);
        public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
            new Dictionary<string, TypeConverter<Value, Value>>();
    }
}
