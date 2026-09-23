using Datasources;
using Datasources.Sqlite;
using static Datasources.Cmp;

namespace Datasources.Sqlite.Tests;

public class ReadonlyRepositoryIntegrationTests
{
    static readonly Guid Ada = Guid.Parse("11111111-1111-1111-1111-111111111111");
    static readonly Guid Grace = Guid.Parse("22222222-2222-2222-2222-222222222222");
    static readonly Guid Lin = Guid.Parse("33333333-3333-3333-3333-333333333333");

    sealed class User
    {
        public int id { get; set; }
        public string username { get; set; } = "";
        public bool is_enabled { get; set; }
    }

    sealed class UserAccount
    {
        public Guid Key { get; set; }
        public string Login { get; set; } = "";
        public bool Active { get; set; }
        public DateTime Opened { get; set; }
    }

    [Fact]
    public async Task ReadsUsersByIdAndByUsernameAndEnabled()
    {
        var connection = await Open("""
            CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL
            );
            INSERT INTO users (id, username, is_enabled) VALUES
                (1, 'ada', 1),
                (2, 'grace', 0);
            """);
        var repo = new ReadonlyRepository<User>(connection, new ReadonlyRepositoryOptions<User>
        {
            EntityMapper = new PluralizerMapper(),
        });

        var byId = await repo.ReadSingle(new WhereClause("id", eq, 1));
        var disabled = new List<User>();
        await foreach (var row in repo.Read(new WhereClause("username", eq, "grace").And("is_enabled", eq, 0)))
            disabled.Add(row);

        Assert.Equal("ada", byId!.username);
        Assert.True(byId.is_enabled);
        var grace = Assert.Single(disabled);
        Assert.Equal(2, grace.id);
        Assert.False(grace.is_enabled);
    }

    [Fact]
    public async Task ReadsUserByGuidUsernameAndCreated()
    {
        var connection = await Open($"""
            CREATE TABLE user (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                is_enabled INTEGER NOT NULL,
                created TEXT NOT NULL
            );
            INSERT INTO user (id, username, is_enabled, created) VALUES

                ('{Ada}', 'ada', 1, '2020-01-01T00:00:00.000Z'),
                ('{Grace}', 'grace', 1, '2024-06-01T00:00:00.000Z'),
                ('{Lin}', 'lin', 0, '2024-06-01T00:00:00.000Z');
            """);
        var repo = new ReadonlyRepository<UserAccount>(connection, new ReadonlyRepositoryOptions<UserAccount>
        {
            Entity = "user",
            ColumnMappings = new DictionaryMapper<string>(new Dictionary<string, string>
            {
                ["Key"] = "id",
                ["Login"] = "username",
                ["Active"] = "is_enabled",
                ["Opened"] = "created",
            }),
            RowTypeConverters = new SqliteDriver().RowTypeConverters,
        });

        var opened = new DateTime(2024, 1, 1, 0, 0, 0, DateTimeKind.Utc);
        var byId = await repo.ReadSingle(new WhereClause("Key", eq, Ada));
        var byLogin = new List<UserAccount>();
        await foreach (var row in repo.Read(new WhereClause("Login", eq, "ada").And("Active", eq, true)))
            byLogin.Add(row);
        var later = new List<UserAccount>();
        await foreach (var row in repo.Read(new WhereClause("Active", eq, true).And("Opened", gt, opened)))
            later.Add(row);

        Assert.Equal("ada", byId!.Login);
        Assert.Equal(Ada, byId.Key);
        Assert.True(byId.Active);
        Assert.Equal(2020, byId.Opened.Year);
        Assert.Equal("ada", Assert.Single(byLogin).Login);
        Assert.Equal("grace", Assert.Single(later).Login);
    }

    static async Task<SqliteConnection> Open(string sql)
    {
        var connection = new SqliteConnection(new SqliteConnectionOptions { Filename = ":memory:" });
        await connection.Open();
        var command = connection.CreateCommand();
        foreach (var statement in sql.Split(';', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries))
            await command.Execute(statement);
        return connection;
    }
}
