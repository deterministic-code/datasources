using System.Text.Json.Nodes;
using Datasources;
using Datasources.Json;

namespace Datasources.Json.Tests;

public class JsonTests
{
    public JsonTests()
    {
        DatasourceFactory.Reset();
        DatasourceFactory.Register("json", new JsonProvider());
    }

    [Fact]
    public void FilterHelpers()
    {
        Assert.Empty(JsonCommand.ParseFilter(null));
        Assert.Empty(JsonCommand.ParseFilter(""));
        Assert.Equal(new Value.Int(1), JsonCommand.ParseFilter("{\"id\":1}")["id"]);
        Assert.Throws<DatasourceException>(() => JsonCommand.ParseFilter("[]"));
        Assert.Throws<DatasourceException>(() => JsonCommand.ParseFilter("1"));
        Assert.False(JsonCommand.Matches(new JsonArray(), new Row()));
        Assert.True(JsonCommand.Matches(new JsonObject { ["id"] = 1 }, new Row()));
    }

    [Fact]
    public async Task FileStoreAndConnection()
    {
        var store = new MemoryFileStore();
        await Assert.ThrowsAsync<DatasourceException>(() => store.Read("a.json"));
        Assert.False(await store.Exists("a.json"));
        await store.Write("a.json", "[{\"id\":2}]");
        Assert.True(await store.Exists("a.json"));

        var existing = new JsonConnection(new JsonConnectionOptions { Path = "a.json", FileStore = store });
        await existing.Open();
        Assert.Single(existing.GetCollection());
        await existing.Open();

        var missing = new JsonConnection(new JsonConnectionOptions { Path = "none.json", FileStore = new MemoryFileStore() });
        await missing.Open();
        Assert.Empty(missing.GetCollection());

        var memory = new JsonConnection(new JsonConnectionOptions { Data = new JsonArray { new JsonObject { ["id"] = 1 } } });
        await memory.Open();
        await memory.Close();

        var collection = new JsonConnection(new JsonConnectionOptions { Collection = "users" });
        await collection.Open();
        collection.Data = JsonValue.Create((string?)null)!;
        collection.GetCollection();
        collection.Data = new JsonArray();
        collection.GetCollection();
        collection.Data = new JsonObject { ["users"] = "bad" };
        collection.GetCollection();

        var plain = new JsonConnection();
        await plain.Open();
        plain.Data = new JsonObject();
        plain.GetCollection();
        await plain.Close();
        Assert.Throws<DatasourceException>(() => plain.EnsureOpen());
        await Assert.ThrowsAsync<DatasourceException>(() => new JsonConnection().StartTransaction());
    }

    [Fact]
    public async Task TransactionsAndCrud()
    {
        var store = new MemoryFileStore();
        var conn = new JsonConnection(new JsonConnectionOptions
        {
            Path = "db.json",
            FileStore = store,
            Data = new JsonArray { new JsonObject { ["id"] = 1, ["name"] = "a" } },
        });
        await conn.Open();
        var txn = await conn.StartTransaction();
        conn.GetCollection().Add(new JsonObject { ["id"] = 2 });
        await conn.Persist();
        Assert.False(store.Files.ContainsKey("db.json"));
        await txn.Commit();
        Assert.True(store.Files.ContainsKey("db.json"));

        var mem = new JsonConnection(new JsonConnectionOptions { Data = new JsonArray() });
        await mem.Open();
        var t = await mem.StartTransaction();
        await t.Abort();
        var t2 = await mem.StartTransaction();
        await t2.Commit();

        var repo = new CRUDRepository<Dictionary<string, Value>>(conn);
        var command = new JsonCommand(conn);
        Assert.NotNull(await repo.ReadSingle());
        Assert.Null(await repo.ReadSingle(new SelectStatement("{\"id\":9}")));
        Assert.NotEmpty(await repo.Read(new SelectStatement("{\"id\":1}")).ToListAsync());
        Assert.NotNull(await repo.ReadSingle(new WhereText("{\"id\":1}")));
        Assert.NotEmpty(await repo.Read(new WhereText("{\"id\":1}")).ToListAsync());
        await repo.Create(new Row { ["id"] = 4, ["name"] = "d" });
        await repo.Update(new Row { ["id"] = 4, ["name"] = "D" });
        await repo.Update(new Row { ["name"] = "Z" }, new WhereText("{\"id\":4}"));
        await repo.Delete(new Row { ["id"] = 4 });
        await command.Execute("insert", new[] { new Parameter("id", 5) });
        await command.Execute("update", new[] { new Parameter("id", 5), new Parameter("$where", "{\"id\":5}") });
        await command.Execute("delete", new[] { new Parameter("id", 5) });
        await command.Execute("delete");
        await Assert.ThrowsAsync<DatasourceException>(() => command.Execute("nope"));
        await Assert.ThrowsAsync<DatasourceException>(() => command.Execute(""));
        await command.ExecuteReader("", new[] { new Parameter("name", "a") });

        var closed = new JsonConnection();
        await Assert.ThrowsAsync<DatasourceException>(() => new JsonCommand(closed).ExecuteReader(""));
        await Assert.ThrowsAsync<DatasourceException>(() => new CRUDRepository<Dictionary<string, Value>>(closed).Create(new Row()));
        await Assert.ThrowsAsync<DatasourceException>(() => new CRUDRepository<Dictionary<string, Value>>(closed).Update(new Row { ["id"] = 1 }));
        await Assert.ThrowsAsync<DatasourceException>(() => new CRUDRepository<Dictionary<string, Value>>(closed).Delete(new Row { ["id"] = 1 }));

        var ds = new Datasource("json", new Of(() => new JsonConnection(new JsonConnectionOptions { Data = new JsonArray() })));
        Assert.Equal("json", ds.Provider);
        var created = await ds.CreateConnection();
        await ((JsonConnection)created).Open();
        var bound = Assert.IsType<JsonCommand>(ds.CreateCommand(created));
        Assert.Same(created, bound.Connection);
        await bound.Execute("insert", [new Parameter("id", 99)]);
        await bound.ExecuteReader("");
        var implicitCommand = Assert.IsType<JsonCommand>(ds.CreateCommand());
        await implicitCommand.Connection.Open();
        await implicitCommand.Execute("insert", [new Parameter("id", 1)]);
        await implicitCommand.ExecuteReader("");
        ds.CreateRepository<Dictionary<string, Value>>(created);
        ds.CreateRepository<Dictionary<string, Value>>();
        Assert.Equal("json", DatasourceFactory.Create(new Dictionary<string, string?> { ["provider"] = "json" }).Provider);

        var fromFile = new JsonConnection(new JsonConnectionOptions
        {
            Path = "c.json",
            Collection = "users",
            FileStore = new MemoryFileStore { Files = { ["c.json"] = "{}" } },
        });
        await fromFile.Open();
    }

    sealed class Of(Func<IConnection> connection) : IDriver
    {
        public IConnection Connection(IReadOnlyDictionary<string, string?>? _) => connection();
        public ICommand Command(IConnection c) => c.CreateCommand();
        public ITransaction Transaction(IConnection c, IsolationLevel isolation = IsolationLevel.Serializable) =>
            new JsonTransaction((JsonConnection)c, isolation);
        public IReadOnlyDictionary<string, TypeConverter<Value, Value>> RowTypeConverters { get; } =
            new Dictionary<string, TypeConverter<Value, Value>>();
    }
}
