using Datasources;
using Datasources.Sqlite;

namespace ContactsCrudEnrichedMediumRest.Tests;

public sealed class NullWriteTests
{
    sealed class ContactStamp
    {
        public int id { get; set; }
        public string name { get; set; } = "";
        public DateTime? created { get; set; }
        public DateTime? updated { get; set; }
        public byte[]? version { get; set; }
    }

    [Fact]
    public async Task OmitsNullCreatedUpdatedAndVersion()
    {
        var connection = new SqliteDriver().Connection(new Dictionary<string, string?> { ["DATASOURCE_URL"] = "sqlite:///:memory:" });
        await connection.Open();
        await connection.CreateCommand().Execute(Native.N("""
            CREATE TABLE contact (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
                updated TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
                version BLOB NOT NULL DEFAULT X'0102'
            )
            """));
        var repo = new CRUDRepository<ContactStamp>(connection, new CRUDRepositoryOptions<ContactStamp> { Entity = "contact" });
        var omitted = new ContactStamp { name = "ada" };
        await repo.Create(omitted);
        var stored = await repo.ReadSingleById(omitted.id);
        omitted.name = "ada-v2";
        await repo.Update(omitted);
        var kept = await repo.ReadSingleById(omitted.id);
        var explicitContact = new ContactStamp
        {
            name = "grace",
            created = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
            updated = new DateTime(2024, 6, 2, 0, 0, 0, DateTimeKind.Utc),
            version = [9, 9]
        };
        await repo.Create(explicitContact);
        var written = await repo.ReadSingleById(explicitContact.id);

        var stamp = new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc);
        Assert.Equal(stamp, stored!.created);
        Assert.Equal(stamp, stored.updated);
        Assert.Equal(new byte[] { 1, 2 }, stored.version);
        Assert.Equal("ada-v2", kept!.name);
        Assert.Equal(stored.created, kept.created);
        Assert.Equal(stored.updated, kept.updated);
        Assert.Equal(stored.version, kept.version);
        Assert.Equal(explicitContact.created, written!.created);
        Assert.Equal(explicitContact.updated, written.updated);
        Assert.Equal(explicitContact.version, written.version);
    }
}
