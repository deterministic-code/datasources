using Datasources;
using Datasources.SqlSpecs;
using Datasources.Sqlite;

namespace Datasources.Sqlite.Tests;

public class NullWriteTests
{
    sealed class Item
    {
        public int Id { get; set; }
        public string Name { get; set; } = "";
        public DateTime? Created { get; set; }
        public byte[]? Version { get; set; }
    }

    [Fact]
    public async Task OmitsNullDatetimeAndVersion()
    {
        var ds = new Datasource("sqlite", new SqliteDriver(), new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        var connection = await ds.CreateConnection();
        await connection.Open();
        await SchemaSpec.Apply(connection, "null-write-item");
        var casing = new ChangeCasingMapper(Casing.Pascal, Casing.Snake);
        var repo = ds.CreateRepository<Item>(connection, new CRUDRepositoryOptions<Item>
        {
            EntityMapper = casing,
            PropertyMappings = casing,
            Ids = ["Id"],
        });
        var omitted = await repo.Create(new Item { Name = "ada" });
        Assert.True(omitted.Id > 0);
        var stored = await repo.ReadSingleById(omitted.Id);
        stored!.Name = "ada-v2";
        var updated = await repo.Update(stored);
        Assert.Equal(1, updated.RowsAffected);
        var kept = await repo.ReadSingleById(omitted.Id);
        var explicitItem = new Item
        {
            Name = "grace",
            Created = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
            Version = [9, 9]
        };
        var writtenCreate = await repo.Create(explicitItem);
        Assert.True(writtenCreate.Id > 0);
        var written = await repo.ReadSingleById(explicitItem.Id);

        Assert.Equal(new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc), stored!.Created);
        Assert.Equal(new byte[] { 1, 2 }, stored.Version);
        Assert.Equal("ada-v2", kept!.Name);
        Assert.Equal(stored.Created, kept.Created);
        Assert.Equal(stored.Version, kept.Version);
        Assert.Equal(explicitItem.Created, written!.Created);
        Assert.Equal(explicitItem.Version, written.Version);
    }

}
