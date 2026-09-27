using Datasources;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class NullWriteTests
{
    sealed class Item
    {
        public int id { get; set; }
        public string name { get; set; } = "";
        public DateTime? created { get; set; }
        public byte[]? version { get; set; }
    }

    [Fact]
    public async Task OmitsNullDatetimeAndVersion()
    {
        var connection = await Open("""
            CREATE TABLE item (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
                version BLOB NOT NULL DEFAULT X'0102'
            );
            """);
        var repo = new CRUDRepository<Item>(connection, new CRUDRepositoryOptions<Item> { Entity = "item" });
        var omitted = new Item { name = "ada" };
        await repo.Create(omitted);
        var stored = await repo.ReadSingleById(omitted.id);
        omitted.name = "ada-v2";
        await repo.Update(omitted);
        var kept = await repo.ReadSingleById(omitted.id);
        var explicitItem = new Item
        {
            name = "grace",
            created = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
            version = [9, 9]
        };
        await repo.Create(explicitItem);
        var written = await repo.ReadSingleById(explicitItem.id);

        Assert.Equal(new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc), stored!.created);
        Assert.Equal(new byte[] { 1, 2 }, stored.version);
        Assert.Equal("ada-v2", kept!.name);
        Assert.Equal(stored.created, kept.created);
        Assert.Equal(stored.version, kept.version);
        Assert.Equal(explicitItem.created, written!.created);
        Assert.Equal(explicitItem.version, written.version);
    }

    static async Task<IConnection> Open(string sql)
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["filename"] = ":memory:" });
        var connection = await ds.CreateConnection();
        await connection.Open();
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(statement);
        return connection;
    }
}
