using Datasources;
using Datasources.Sqlite;

namespace UserRolesSimpleCli.Tests;

public sealed class NullWriteTests
{
    sealed class UserStamp
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public DateTime? created { get; set; }
        public byte[]? updated { get; set; }
    }

    [Fact]
    public async Task OmitsNullCreatedAndUpdated()
    {
        var connection = new SqliteDriver().Connection(new Dictionary<string, string?> { ["filename"] = ":memory:" });
        await connection.Open();
        await connection.CreateCommand().Execute("""
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                created TEXT NOT NULL DEFAULT '2020-01-01T00:00:00.000Z',
                updated BLOB NOT NULL DEFAULT X'0102'
            )
            """);
        var repo = new CRUDRepository<UserStamp>(connection, new CRUDRepositoryOptions<UserStamp> { Entity = "users" });
        var omitted = new UserStamp { username = "ada" };
        await repo.Create(omitted);
        var stored = await repo.ReadSingleById(omitted.id);
        omitted.username = "ada-v2";
        await repo.Update(omitted);
        var kept = await repo.ReadSingleById(omitted.id);
        var explicitUser = new UserStamp
        {
            username = "grace",
            created = new DateTime(2024, 6, 1, 0, 0, 0, DateTimeKind.Utc),
            updated = [9, 9]
        };
        await repo.Create(explicitUser);
        var written = await repo.ReadSingleById(explicitUser.id);

        Assert.Equal(new DateTime(2020, 1, 1, 0, 0, 0, DateTimeKind.Utc), stored!.created);
        Assert.Equal(new byte[] { 1, 2 }, stored.updated);
        Assert.Equal("ada-v2", kept!.username);
        Assert.Equal(stored.created, kept.created);
        Assert.Equal(stored.updated, kept.updated);
        Assert.Equal(explicitUser.created, written!.created);
        Assert.Equal(explicitUser.updated, written.updated);
    }
}
